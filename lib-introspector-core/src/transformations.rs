/// Source code transformation functions
/// 
/// This module contains all the transformation functions extracted from build.rs
/// that modify Rust source code to make it parseable and compilable.

/// Apply a transformation by name to source content
/// 
/// # Arguments
/// * `content` - The source code to transform
/// * `name` - The transformation name to apply
/// 
/// # Supported Transformations
/// - "add_prelude" - Adds standard prelude imports
/// - "fix_file_paths" - Fixes relative file paths  
/// - "fix_env_vars" - Fixes environment variable references
/// - "fix_attribute_spacing" - Normalizes attribute spacing
/// - "remove_crate_attrs" - Removes problematic crate attributes
/// - "strip_incomplete_docs" - Removes incomplete doc comments
pub fn apply_transformation_by_name(content: &str, name: &str) -> String {
    match name {
        "add_prelude" => add_prelude(content),
        "fix_file_paths" => fix_file_paths(content),
        "fix_env_vars" => fix_env_vars(content),
        "fix_attribute_spacing" => fix_attribute_spacing(content),
        "remove_crate_attrs" => remove_crate_attrs(content),
        "strip_incomplete_docs" => strip_incomplete_docs(content),
        _ => content.to_string(),
    }
}

/// Add standard Rust prelude to source code
pub fn add_prelude(content: &str) -> String {
    // Always put feature at the very beginning
    format!("#![feature(try_blocks)]\n{}", content)
}

/// Fix relative file paths in source code
pub fn fix_file_paths(content: &str) -> String {
    content
        .replace("../rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/")
        .replace("../submodules/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/")
        .replace("submodules/rust/", "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/")
}

/// Fix environment variable references
pub fn fix_env_vars(content: &str) -> String {
    // Placeholder - implement based on build.rs
    content.to_string()
}

/// Fix attribute spacing issues (# [attr] -> #[attr])
pub fn fix_attribute_spacing(content: &str) -> String {
    // Placeholder - implement based on build.rs  
    content.to_string()
}

/// Remove problematic crate attributes
pub fn remove_crate_attrs(content: &str) -> String {
    // Placeholder - implement based on build.rs
    content.to_string()
}

/// Strip incomplete doc comments that break parsing
pub fn strip_incomplete_docs(content: &str) -> String {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Keep complete doc comments (end with . or !)
            if trimmed.starts_with("///") {
                trimmed.ends_with('.') || trimmed.ends_with('!') || trimmed == "///"
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Process content through the full transformation pipeline
/// 
/// This is the main processing function that applies all transformations
/// in sequence and returns the final result.
pub fn process_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let transformations = vec![
        "strip_incomplete_docs",
        "add_prelude",
        "fix_file_paths", 
        "fix_env_vars",
        "fix_attribute_spacing",
        "remove_crate_attrs",
    ];

    let mut current = content.to_string();
    for transform_name in &transformations {
        current = apply_transformation_by_name(&current, transform_name);
    }

    Ok(current)
}
