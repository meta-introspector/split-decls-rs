use syn::{parse_file, File};
use quote::quote;
use crate::track_transform;

/// Safe string transformation that validates AST integrity
pub fn safe_string_transform<F>(content: &str, transform_id: &str, description: &str, transform_fn: F) -> String 
where
    F: Fn(&str) -> String,
{
    track_transform!(content, transform_id, description, |content: &str| {
        // Step 1: Extract original AST
        let original_ast = match parse_file(content) {
            Ok(ast) => ast,
            Err(_) => return content.to_string(), // Can't parse original, return unchanged
        };

        // Step 2: Convert AST to string
        let ast_string = quote!(#original_ast).to_string();

        // Step 3: Apply string transformation
        let transformed_string = transform_fn(&ast_string);

        // Step 4: Parse transformed string back to AST
        let transformed_ast = match parse_file(&transformed_string) {
            Ok(ast) => ast,
            Err(_) => {
                // Transformation corrupted the AST, return original
                eprintln!("⚠️  Transformation '{}' corrupted AST, reverting", transform_id);
                return content.to_string();
            }
        };

        // Step 5: Return validated transformed code
        quote!(#transformed_ast).to_string()
    })
}

/// Compare two ASTs and generate semantic patch information
pub fn create_semantic_patch(original: &File, transformed: &File) -> SemanticPatch {
    SemanticPatch {
        items_added: count_items(transformed) - count_items(original),
        items_removed: count_items(original) - count_items(transformed),
        items_modified: 0, // TODO: implement deep comparison
        is_valid: true,
    }
}

#[derive(Debug)]
pub struct SemanticPatch {
    pub items_added: i32,
    pub items_removed: i32,
    pub items_modified: i32,
    pub is_valid: bool,
}

fn count_items(file: &File) -> i32 {
    file.items.len() as i32
}
