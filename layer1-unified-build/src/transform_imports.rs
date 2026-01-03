use syn::{parse_file, File, Item, ItemUse, UseTree, Path};
use quote::quote;
use crate::track_transform;

/// Semantically add imports to the correct location in sorted order
pub fn add_import_semantic(content: &str, import_path: &str) -> String {
    track_transform!(content, "ADD_IMPORT_SEMANTIC", &format!("Add import: {}", import_path), |content: &str| {
        match parse_file(content) {
            Ok(mut file) => {
                // Check if import already exists
                if import_exists(&file, import_path) {
                    return content.to_string(); // Already exists, no change needed
                }

                // Create new import item
                let new_import = create_import_item(import_path);
                
                // Find correct insertion point (after existing imports, sorted)
                let insert_pos = find_import_insertion_point(&file, import_path);
                
                // Insert the import
                file.items.insert(insert_pos, new_import);
                
                quote!(#file).to_string()
            }
            Err(_) => content.to_string() // Can't parse, return unchanged
        }
    })
}

/// Check if an import already exists in the file
fn import_exists(file: &File, import_path: &str) -> bool {
    file.items.iter().any(|item| {
        if let Item::Use(use_item) = item {
            use_tree_contains_path(&use_item.tree, import_path)
        } else {
            false
        }
    })
}

/// Check if a use tree contains the given import path
fn use_tree_contains_path(tree: &UseTree, target_path: &str) -> bool {
    match tree {
        UseTree::Path(path) => {
            let current_path = quote!(#path).to_string();
            current_path.contains(target_path) || use_tree_contains_path(&path.tree, target_path)
        }
        UseTree::Name(name) => {
            quote!(#name).to_string() == target_path
        }
        UseTree::Group(group) => {
            group.items.iter().any(|item| use_tree_contains_path(item, target_path))
        }
        UseTree::Glob(_) => false, // Glob imports don't match specific paths
        UseTree::Rename(rename) => {
            quote!(#rename.ident).to_string() == target_path
        }
    }
}

/// Create a new import item from a path string
fn create_import_item(import_path: &str) -> Item {
    let use_stmt: ItemUse = syn::parse_str(&format!("use {};", import_path))
        .expect("Failed to parse import path");
    Item::Use(use_stmt)
}

/// Find the correct insertion point for a new import (sorted order)
fn find_import_insertion_point(file: &File, import_path: &str) -> usize {
    let mut last_import_pos = 0;
    
    for (i, item) in file.items.iter().enumerate() {
        match item {
            Item::Use(use_item) => {
                last_import_pos = i + 1;
                
                // Check if we should insert before this import (alphabetical order)
                let existing_path = quote!(#use_item.tree).to_string();
                if import_path < existing_path.as_str() {
                    return i;
                }
            }
            // Stop at first non-use, non-attribute item
            Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | Item::Impl(_) | Item::Mod(_) => {
                break;
            }
            _ => continue,
        }
    }
    
    last_import_pos
}

/// Add multiple imports semantically
pub fn add_imports_semantic(content: &str, imports: &[&str]) -> String {
    let mut result = content.to_string();
    
    // Sort imports to ensure consistent ordering
    let mut sorted_imports = imports.to_vec();
    sorted_imports.sort();
    
    for import in sorted_imports {
        result = add_import_semantic(&result, import);
    }
    
    result
}
