use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item};
use quote::ToTokens;
use serde::Deserialize;
use rayon::prelude::*;
use walkdir::WalkDir;

fn find_rust_files(crate_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(crate_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            // Skip output directories
            if path.to_string_lossy().contains("output2") || 
               path.to_string_lossy().contains("enhanced_output") {
                return false;
            }
            path.extension().map_or(false, |ext| ext == "rs")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn find_crate_directories() -> Result<Vec<PathBuf>> {
    let mut crate_dirs = Vec::new();
    
    // Essential crates for split-decls-rs
    let essential_crates = [
        "..",  // split-decls-rs itself
        "../crates/split-decls-types",
        "../crates/cargo-toml-generator-types", 
        "../crates/cargo-toml-generator-macros",
        "../crates/cargo-toml-parts",
        "../crates/pagerank_rs",
        "../crates/introspector_decl2_macros",
        "../crates/introspector_decl_common",
        "../crates/introspector_decl_core",
        "../crates/introspector_macro_helpers",
    ];
    
    for crate_path in &essential_crates {
        let path = Path::new(crate_path);
        if path.join("Cargo.toml").exists() && path.join("src").exists() {
            crate_dirs.push(path.to_path_buf());
        }
    }
    
    Ok(crate_dirs)
}

fn split_crate(crate_path: &Path) -> Result<()> {
    let rust_files = find_rust_files(crate_path);
    if rust_files.is_empty() {
        return Ok(());
    }
    
    let crate_name = if crate_path.to_string_lossy().ends_with("..") {
        "split-decls-rs"  // Use proper name for parent directory
    } else {
        crate_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
    };
    
    println!("🔄 Processing {}: {} files", crate_name, rust_files.len());
    
    let output_dir = Path::new("../output2").join(format!("wrapped-{}/src/decls", crate_name));
    fs::create_dir_all(&output_dir)?;
    
    let mut total_count = 0;
    
    for rust_file in rust_files {
        if let Ok(content) = fs::read_to_string(&rust_file) {
            if let Ok(parsed) = syn::parse_file(&content) {
                let module_name = if rust_file.to_string_lossy().contains("/bin/") {
                    // Extract binary name from path like "src/bin/enhanced_wrapper.rs"
                    rust_file.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                } else {
                    rust_file.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("lib")
                };
                
                for (i, item) in parsed.items.iter().enumerate() {
                    let (decl_type, name) = match item {
                        Item::Fn(f) => ("fn", f.sig.ident.to_string()),
                        Item::Struct(s) => ("struct", s.ident.to_string()),
                        Item::Enum(e) => ("enum", e.ident.to_string()),
                        Item::Trait(t) => ("trait", t.ident.to_string()),
                        Item::Impl(_) => ("impl", format!("impl_{}", i)),
                        Item::Const(c) => ("const", c.ident.to_string()),
                        Item::Static(s) => ("static", s.ident.to_string()),
                        Item::Type(t) => ("type", t.ident.to_string()),
                        Item::Union(u) => ("union", u.ident.to_string()),
                        _ => continue,
                    };
                    
                    let wrapped = format!(
                        "use serde::{{Deserialize, Serialize}};\nuse std::collections::HashMap;\n\nmkdecl{}! {{\n{}\n}}",
                        decl_type,
                        item.to_token_stream()
                    );
                    
                    let decl_size = wrapped.len();
                    let complexity = match decl_size {
                        0..=200 => 1,
                        201..=400 => 2,
                        401..=600 => 3,
                        601..=800 => 4,
                        801..=1000 => 5,
                        1001..=1300 => 6,
                        1301..=1700 => 7,
                        1701..=2500 => 8,
                        2501..=4000 => 9,
                        _ => 10,
                    };
                    
                    let module_dir = output_dir.join(module_name).join(decl_type).join(complexity.to_string());
                    fs::create_dir_all(&module_dir)?;
                    
                    let file_path = module_dir.join(format!("{}.rs", name));
                    fs::write(file_path, wrapped)?;
                    total_count += 1;
                }
            }
        }
    }
    
    if total_count > 0 {
        // Generate new lib.rs for wrapped crate
        let wrapped_lib_dir = Path::new("../output2").join(format!("wrapped-{}/src", crate_name));
        fs::create_dir_all(&wrapped_lib_dir)?;
        
        let new_lib = "pub mod decls;\npub use decls::*;\n";
        fs::write(wrapped_lib_dir.join("lib.rs"), new_lib)?;
        
        println!("✅ Split {} items from {}", total_count, crate_name);
    }
    
    Ok(())
}

fn main() -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build_global()?;
    
    let crate_dirs = find_crate_directories()?;
    
    println!("🚀 Processing {} crate directories with up to 20 crates in parallel", crate_dirs.len());
    
    crate_dirs.par_iter().for_each(|crate_dir| {
        if let Err(e) = split_crate(crate_dir) {
            println!("❌ Failed to process {}: {}", crate_dir.display(), e);
        }
    });
    
    println!("🎉 Done!");
    Ok(())
}
