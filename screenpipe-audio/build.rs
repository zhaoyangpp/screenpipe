#[cfg(target_os = "windows")]
use std::{env, fs, process::Command};
use std::{io::Result, process::Output};

fn main() {
    #[cfg(target_os = "windows")]
    {
        install_onnxruntime();
    }

    if !is_bun_installed() {
        install_bun();
    }
}

fn is_bun_installed() -> bool {
    let output = Command::new("bun").arg("--version").output();

    match output {
        Err(_) => false,
        Ok(output) => output.status.success(),
    }
}

fn run_bun_install_command(command: Result<Output>) {
    match command {
        Err(error) => {
            println!("Failed to install bun: {}", error);
            println!("Please install bun manually.");
        }
        Ok(output) => {
            if output.status.success() {
                println!("Bun installed successfully.");
            } else {
                println!(
                    "Failed to install bun: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                println!("Please install bun manually.");
            }
        }
    }
}

fn install_bun() {
    println!("Installing bun...");

    #[cfg(target_os = "windows")]
    {
        println!("Attempting to install bun using npm...");

        run_bun_install_command(Command::new("npm").args(["install", "-g", "bun"]).output());
    }

    #[cfg(not(target_os = "windows"))]
    {
        run_bun_install_command(
            Command::new("sh")
                .args(["-c", "curl -fsSL https://bun.sh/install | bash"])
                .output(),
        );
    }
}

#[cfg(target_os = "windows")]
fn install_onnxruntime() {
    // Set static CRT for Windows MSVC target
    if env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
        println!("cargo:rustc-env=KNF_STATIC_CRT=1");
        println!("cargo:rustc-flag=-C target-feature=+crt-static");
    }

    let url = "https://github.com/microsoft/onnxruntime/releases/download/v1.19.2/onnxruntime-win-x64-gpu-1.19.2.zip";
    
    // Download onnxruntime zip
    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.bytes() {
            Ok(body) => {
                // Write to file
                if let Err(e) = fs::write("./onnxruntime-win-x64-gpu-1.19.2.zip", &body) {
                    panic!("Failed to write file: {}", e);
                }

                // Unzip the file
                let status = Command::new("unzip")
                    .args(["onnxruntime-win-x64-gpu-1.19.2.zip"])
                    .status()
                    .expect("Failed to execute unzip command");

                if !status.success() {
                    panic!("Failed to install onnxruntime binary");
                }

                // Rename extracted folder
                if let Err(e) = fs::rename(
                    "onnxruntime-win-x64-gpu-1.19.2",
                    "../screenpipe-app-tauri/src-tauri/onnxruntime-win-x64-gpu-1.19.2",
                ) {
                    panic!("Failed to rename folder: {}", e);
                }
            }
            Err(e) => panic!("Failed to read response body: {}", e),
        },
        Err(e) => panic!("Failed to fetch onnxruntime: {}", e),
    }
}

