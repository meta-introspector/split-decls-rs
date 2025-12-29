use syn::{File, Item, UseTree, UsePath, UseGroup, UseGlob, UseRename};
use std::collections::HashSet;
use proc_macro2::TokenStream;

pub fn extract_dependencies_from_macro_wrapped_code(content: &str) -> HashSet<String> {
    let mut deps = HashSet::new();
    
    if let Ok(file) = syn::parse_file(content) {
        extract_deps_from_file(&file, &mut deps);
    }
    
    // Also try to extract from raw text for macro_rules! patterns
    extract_deps_from_text(content, &mut deps);
    
    deps
}

fn extract_deps_from_file(file: &File, deps: &mut HashSet<String>) {
    for item in &file.items {
        extract_deps_from_item(item, deps);
    }
}

fn extract_deps_from_item(item: &Item, deps: &mut HashSet<String>) {
    match item {
        Item::Use(use_item) => {
            extract_deps_from_use_tree(&use_item.tree, deps);
        }
        Item::Macro(macro_item) => {
            extract_deps_from_macro(macro_item, deps);
        }
        Item::Mod(mod_item) => {
            if let Some((_, items)) = &mod_item.content {
                for item in items {
                    extract_deps_from_item(item, deps);
                }
            }
        }
        _ => {}
    }
}

fn extract_deps_from_macro(macro_item: &syn::ItemMacro, deps: &mut HashSet<String>) {
    // Check if this is a deps! macro definition
    if let Some(ident) = &macro_item.ident {
        if ident == "deps" {
            extract_deps_from_macro_body(&macro_item.mac.tokens, deps);
        }
    }
}

fn extract_deps_from_text(content: &str, deps: &mut HashSet<String>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_deps_macro = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Check if we're entering a deps macro
        if trimmed.starts_with("macro_rules! deps") {
            in_deps_macro = true;
            continue;
        }
        
        // Check if we're exiting the macro
        if in_deps_macro && trimmed == "}" {
            in_deps_macro = false;
            continue;
        }
        
        // Extract dependencies from within the deps macro
        if in_deps_macro && trimmed.ends_with("!();") {
            let dep_name = trimmed.trim_end_matches("!();").trim();
            if !dep_name.is_empty() && is_valid_dependency(dep_name) {
                deps.insert(dep_name.to_string());
            }
        }
    }
}

fn extract_deps_from_macro_body(tokens: &TokenStream, deps: &mut HashSet<String>) {
    let token_string = tokens.to_string();
    
    // Extract macro calls like "SomeDep!();"
    for line in token_string.lines() {
        let line = line.trim();
        if line.ends_with("!();") {
            let dep_name = line.trim_end_matches("!();").trim();
            if !dep_name.is_empty() && is_valid_dependency(dep_name) {
                deps.insert(dep_name.to_string());
            }
        }
    }
}

fn extract_deps_from_use_tree(tree: &UseTree, deps: &mut HashSet<String>) {
    match tree {
        UseTree::Path(UsePath { ident, tree, .. }) => {
            let crate_name = ident.to_string();
            if is_external_crate(&crate_name) {
                deps.insert(crate_name);
            }
            extract_deps_from_use_tree(tree, deps);
        }
        UseTree::Group(UseGroup { items, .. }) => {
            for item in items {
                extract_deps_from_use_tree(item, deps);
            }
        }
        UseTree::Glob(UseGlob { .. }) => {}
        UseTree::Name(name) => {
            let crate_name = name.ident.to_string();
            if is_external_crate(&crate_name) {
                deps.insert(crate_name);
            }
        }
        UseTree::Rename(UseRename { ident, .. }) => {
            let crate_name = ident.to_string();
            if is_external_crate(&crate_name) {
                deps.insert(crate_name);
            }
        }
    }
}

fn is_external_crate(name: &str) -> bool {
    !matches!(name, "std" | "core" | "alloc" | "self" | "super" | "crate")
}

fn is_valid_dependency(name: &str) -> bool {
    // Filter out common non-dependency macro calls
    !name.is_empty() && !name.starts_with('_')
}
