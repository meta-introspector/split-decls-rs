#!/usr/bin/env rust-script

use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Generating missing lib.rs files for rustc crates...");
    
    // Find all rustc crate directories that are missing lib.rs
    let rustc_crates = [
        "submodules/rust/compiler/rustc_errors",
        "submodules/rust/compiler/rustc_data_structures", 
        "submodules/rust/compiler/rustc_session",
        "submodules/rust/compiler/rustc_middle",
        "submodules/rust/compiler/rustc_hir",
        "submodules/rust/compiler/rustc_ast",
        "submodules/rust/compiler/rustc_span",
        "submodules/rust/compiler/rustc_target",
        "submodules/rust/compiler/rustc_metadata",
        "submodules/rust/compiler/rustc_infer",
        "submodules/rust/compiler/rustc_trait_selection",
        "submodules/rust/compiler/rustc_ty_utils",
        "submodules/rust/compiler/rustc_monomorphize",
        "submodules/rust/compiler/rustc_const_eval",
    ];
    
    for crate_path in &rustc_crates {
        let src_dir = format!("{}/src", crate_path);
        let lib_path = format!("{}/lib.rs", src_dir);
        
        if Path::new(&src_dir).exists() && !Path::new(&lib_path).exists() {
            println!("📦 Creating lib.rs for: {}", crate_path);
            
            // Find all .rs files in the src directory
            let mut modules = Vec::new();
            if let Ok(entries) = fs::read_dir(&src_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(name) = path.file_stem() {
                            if let Some(name_str) = name.to_str() {
                                if path.extension().map_or(false, |ext| ext == "rs") 
                                   && name_str != "lib" 
                                   && name_str != "main" {
                                    modules.push(name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
            
            // Generate lib.rs content
            let mut lib_content = String::new();
            lib_content.push_str("// Generated lib.rs for rustc crate\n");
            lib_content.push_str("// This file re-exports all modules in this crate\n\n");
            
            // Add feature flags commonly needed by rustc
            lib_content.push_str("#![feature(rustc_private)]\n");
            lib_content.push_str("#![allow(internal_features)]\n\n");
            
            // Add module declarations and re-exports
            for module in &modules {
                lib_content.push_str(&format!("pub mod {};\n", module));
            }
            
            lib_content.push_str("\n// Re-export everything\n");
            for module in &modules {
                lib_content.push_str(&format!("pub use {}::*;\n", module));
            }
            
            // Write the lib.rs file
            fs::write(&lib_path, lib_content)?;
            println!("✅ Created: {} with {} modules", lib_path, modules.len());
        }
    }
    
    println!("🎉 Finished generating lib.rs files!");
    Ok(())
}
