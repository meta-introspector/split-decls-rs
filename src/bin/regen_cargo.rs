use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use crate::{generate_new_cargotoml, patch_config};
use std::fs;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "regen-cargo")]
#[command(about = "Regenerate only Cargo.toml files for existing wrapped workspace")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output directory (default: output2)
    #[arg(short, long, default_value = "output2")]
    output_dir: PathBuf,

    /// Dry run mode
    #[arg(short, long)]
    dry_run: bool,

    /// Build after regenerating
    #[arg(short, long)]
    build: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Regenerating Cargo.toml files in {}", cli.output_dir.display());
    }

    let patch_config = if std::path::Path::new("patch.toml").exists() {
        patch_config::PatchConfig::load_from_file(&PathBuf::from("patch.toml"))?
    } else {
        patch_config::PatchConfig::default()
    };

    let global_config = &crate::config_macros::GLOBAL_CONFIG.lock().unwrap();
    
    // Generate workspace Cargo.toml first
    let mut workspace_content = String::new();
    workspace_content.push_str("[workspace]\nresolver = \"2\"\nmembers = [\n");
    
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.starts_with("wrapped-") {
                    workspace_content.push_str(&format!("    \"{}\",\n", name));
                }
            }
        }
    }
    
    workspace_content.push_str("]\n\n[workspace.dependencies]\n");
    
    for crate_name in &global_config.wrapping.crates {
        if let Some(path_override) = global_config.crate_path_overrides.as_ref().and_then(|overrides| overrides.get(crate_name)) {
            if path_override.to_string_lossy() == "*" {
                workspace_content.push_str(&format!("{} = {{ version = \"*\" }}\n", crate_name));
            } else {
                let path_str = path_override.to_string_lossy();
                let relative_path = if path_str.contains("/submodules/") {
                    let parts: Vec<&str> = path_str.split("/submodules/").collect();
                    format!("../submodules/{}", parts[1])
                } else {
                    format!("../{}", crate_name)
                };
                workspace_content.push_str(&format!("{} = {{ path = \"{}\" }}\n", crate_name, relative_path));
            }
        }
    }
    
    fs::write(cli.output_dir.join("Cargo.toml"), workspace_content)?;
    let mut updated_crates = Vec::new();

    // Find all wrapped-* directories and regenerate their Cargo.toml files
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("wrapped-"))
            .unwrap_or(false) 
        {
            let crate_name = path.file_name().unwrap().to_str().unwrap()
                .strip_prefix("wrapped-").unwrap();
            
            if cli.verbose {
                println!("Updating Cargo.toml for {}", crate_name);
            }

            // Find original crate path
            let original_path = PathBuf::from(crate_name);
            if !original_path.exists() {
                let submodule_path = PathBuf::from("submodules").join(crate_name);
                if submodule_path.exists() {
                    let original_cargo = submodule_path.join("Cargo.toml");
                    let output_cargo = path.join("Cargo.toml");
                    
                    if !cli.dry_run {
                        generate_new_cargotoml::generate_new_cargotoml(
                            &original_cargo,
                            &output_cargo,
                            &submodule_path,
                            global_config,
                            &patch_config,
                            cli.dry_run,
                        )?;
                        
                        // Fix alloc alias - replace with rustc-std-workspace-alloc
                        let cargo_content = fs::read_to_string(&output_cargo)?;
                        let fixed_content = cargo_content
                            .replace("alloc]\nworkspace = true", "rustc-std-workspace-alloc]\nworkspace = true")
                            .replace("[dependencies.alloc]", "[dependencies.rustc-std-workspace-alloc]")
                            .replace("[dev-dependencies.alloc]", "[dev-dependencies.rustc-std-workspace-alloc]")
                            .replace("[build-dependencies.alloc]", "[build-dependencies.rustc-std-workspace-alloc]");
                        fs::write(&output_cargo, fixed_content)?;
                        
                        updated_crates.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    if cli.verbose {
        println!("✅ Cargo.toml regeneration complete");
    }

    if cli.build && !cli.dry_run {
        if cli.verbose {
            println!("Building updated crates...");
        }
        
        let mut error_count = 0;
        for crate_path in updated_crates {
            let output = Command::new("cargo")
                .args(&["check", "--quiet"])
                .current_dir(&crate_path)
                .output();
                
            match output {
                Ok(result) if !result.status.success() => {
                    error_count += 1;
                    let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                    println!("❌ {}: {}", crate_name, String::from_utf8_lossy(&result.stderr).lines().next().unwrap_or("Build failed"));
                }
                Err(e) => {
                    error_count += 1;
                    let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                    println!("❌ {}: {}", crate_name, e);
                }
                _ => {} // Success, no output
            }
        }
        
        if error_count > 0 {
            println!("❌ {} crates failed to build", error_count);
        } else if cli.verbose {
            println!("✅ All crates built successfully");
        }
    }

    Ok(())
}
