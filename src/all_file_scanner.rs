use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item};
use quote::ToTokens;

/// Scan all Rust source files in a crate and extract functions
pub fn scan_all_rust_files(crate_path: &Path) -> Result<Vec<(PathBuf, Vec<Item>)>> {
    let mut all_files = Vec::new();
    
    // Scan src/ directory recursively
    let src_dir = crate_path.join("src");
    if src_dir.exists() {
        scan_rust_files_recursive(&src_dir, &mut all_files)?;
    }
    
    Ok(all_files)
}

fn scan_rust_files_recursive(dir: &Path, files: &mut Vec<(PathBuf, Vec<Item>)>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            scan_rust_files_recursive(&path, files)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(parsed) = syn::parse_file(&content) {
                    let functions: Vec<Item> = parsed.items.into_iter()
                        .filter(|item| matches!(item, Item::Fn(_)))
                        .collect();
                    
                    if !functions.is_empty() {
                        files.push((path, functions));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Generate wrapped declaration with file origin header
pub fn generate_wrapped_decl_with_origin(
    item: &Item,
    original_file: &Path,
    crate_name: &str,
) -> String {
    let file_name = original_file.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    let item_name = match item {
        Item::Fn(func) => func.sig.ident.to_string(),
        _ => "unknown".to_string(),
    };
    
    format!(
        "// Generated from: {}\n// Original file: {}\n// Function: {}\n\nuse proc_macro::TokenStream;\nuse quote::quote;\nuse syn::*;\nuse std::path::{{Path, PathBuf}};\nuse anyhow::{{Context, Result}};\nuse split_decls_types::SplitDeclsConfig;\npub use extracted_decl::*;\npub use process_crate::process_crate;\npub use process_crates_in_path::process_crates_in_path;\npub use generate_wrapped_workspace::generate_wrapped_workspace;\nprelude!{{}}\n\n#[decl_{}_{}]\n{}",
        original_file.display(),
        original_file.display(),
        item_name,
        crate_name,
        file_name,
        item.to_token_stream()
    )
}
