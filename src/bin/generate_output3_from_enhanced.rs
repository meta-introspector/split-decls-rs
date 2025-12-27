use std::fs;
use anyhow::Result;
use split_decls_types::SplitDeclsConfig;
use crate::*;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let scan_root = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        // For output3 generation, we want to use the original submodules as source
        // and apply the enhanced declarations from enhanced_output
        let candidates = ["../../submodules", "../../", "output2"];
        let mut found_root = None;
        
        for candidate in &candidates {
            let path = Path::new(candidate);
            if path.exists() {
                // Check if this looks like a valid source directory
                if *candidate == "../../submodules" || *candidate == "../../" {
                    // Check for some known crates
                    let test_crates = ["addr2line", "adler2", "ahash"];
                    let has_crates = test_crates.iter().any(|crate_name| {
                        path.join(crate_name).join("Cargo.toml").exists()
                    });
                    if has_crates {
                        found_root = Some(path);
                        break;
                    }
                } else if *candidate == "output2" {
                    // Check if it has wrapped crates as fallback
                    let has_wrapped_crates = fs::read_dir(path)
                        .map(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .any(|e| e.file_name().to_string_lossy().starts_with("wrapped-"))
                        })
                        .unwrap_or(false);
                    
                    if has_wrapped_crates {
                        found_root = Some(path);
                        break;
                    }
                }
            }
        }
        
        match found_root {
            Some(root) => {
                println!("Using input source: {}", root.display());
                root.to_path_buf()
            },
            None => {
                eprintln!("No valid input directory found. Tried: {:?}", candidates);
                eprintln!("Expected to find original crates in ../../submodules/ or wrapped crates in output2/");
                std::process::exit(1);
            }
        }
    };
    
    println!("Generating output3 from: {}", scan_root.display());
    
    // Load the actual configuration from split-decls-rs.toml
    let config = if Path::new("split-decls-rs.toml").exists() {
        println!("Loading configuration from split-decls-rs.toml");
        SplitDeclsConfig::load_from_file(Path::new("split-decls-rs.toml"))?
    } else {
        println!("Warning: split-decls-rs.toml not found, using default config");
        SplitDeclsConfig::default()
    };
    
    println!("Configuration loaded with {} crates to wrap", config.wrapping.crates.len());
    
    let patch_config = patch_config::PatchConfig::default();
    let output_dir = PathBuf::from("output3");
    
    let module_not_found_errors = generate_wrapped_workspace(
        &output_dir,
        &patch_config,
        &config,
        &scan_root,
        false, // dry_run
        true,  // verbose
        false  // cargo_only
    )?;
    
    println!("Bootstrap completed with {} errors!", module_not_found_errors.len());
    
    // Check what was generated
    if output_dir.exists() {
        let entries = fs::read_dir(&output_dir)?;
        let wrapped_count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("wrapped-"))
            .count();
        
        println!("Generated {} wrapped crates in output3", wrapped_count);
        
        // If we have wrapped crates, try to test one
        if wrapped_count > 0 {
            println!("Output3 generation successful!");
            
            // Check if we have the self-referential wrapped-split-decls-rs
            let wrapped_self = output_dir.join("wrapped-split-decls-rs");
            if wrapped_self.exists() {
                println!("✅ Self-referential wrapped-split-decls-rs generated - recursive capability achieved!");
            }
        } else {
            println!("⚠️  Warning: No wrapped crates were generated");
        }
    } else {
        println!("❌ Error: Output directory was not created");
    }
    
    Ok(())
}
