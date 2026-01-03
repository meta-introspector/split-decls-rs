// Individual transformation functions for rustc codebase processing
// Each function is testable, auditable, and can be used independently

use crate::transformation_tracker::*;

/// Add compiler bootstrap feature flags to lib.rs files
pub fn add_bootstrap_features(content: &str) -> String {
    track_transform!(content, "BOOTSTRAP_FEATURES", "Add compiler bootstrap feature flags", |content: &str| {
        let bootstrap_features = vec![
            "rustc_private",
            "never_type", 
            "rustc_attrs",
            "lang_items",
            "optimize_attribute",
            "allocator_api",
            "maybe_uninit_write_slice",
            "panic_can_unwind", 
            "extend_one",
            "proc_macro_diagnostic",
            "proc_macro_span",
            "proc_macro_tracked_env",
            "track_path",
            "proc_macro_expand",
            "proc_macro_internals",
            "proc_macro_value",
            "allow_internal_unstable",
            "allow_internal_unsafe",
            "staged_api",
            "if_let_guard",
            "core_io_borrowed_buf",
            "read_buf",
            "map_try_insert",
            "array_windows",
        ];

        let mut result = content.to_string();
        
        // Find insertion point after existing features
        let lines: Vec<&str> = result.lines().collect();
        let mut insert_pos = 0;
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("#![feature(") {
                insert_pos = i + 1;
            } else if line.starts_with("#![") && insert_pos == 0 {
                insert_pos = i;
                break;
            }
        }
        
        // Add missing features
        let mut new_lines = lines;
        for feature in bootstrap_features {
            let feature_line = format!("#![feature({})]", feature);
            if !result.contains(&feature_line) {
                new_lines.insert(insert_pos, Box::leak(feature_line.into_boxed_str()));
                insert_pos += 1;
            }
        }
        
        new_lines.join("\n")
    })
}

/// Strip malformed doc attributes that cause compilation errors
pub fn strip_malformed_doc_attributes(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_malformed_doc = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Detect malformed doc attributes
        if trimmed.starts_with("#![doc(") && !trimmed.ends_with(")]") {
            in_malformed_doc = true;
            continue;
        }
        
        if in_malformed_doc {
            if trimmed.ends_with(")]") {
                in_malformed_doc = false;
            }
            continue;
        }
        
        result.push(line);
    }
    
    result.join("\n")
}

/// Add missing type definitions for rustc core types
pub fn add_missing_type_definitions(content: &str, file_path: &str) -> String {
    let mut result = content.to_string();
    
    match file_path {
        path if path.contains("def_id.rs") => {
            if !result.contains("pub type CrateNum") {
                result = add_after_imports(&result, "
// Add missing types with proper implementations
rustc_index::newtype_index! {
    pub struct CrateNum {}
}

rustc_index::newtype_index! {
    pub struct DefIndex {}
}

pub const CRATE_DEF_INDEX: DefIndex = DefIndex::ZERO;");
            }
        }
        path if path.contains("symbol.rs") => {
            if !result.contains("pub type SymbolIndex") {
                result = add_after_imports(&result, "
// Add missing types
rustc_index::newtype_index! {
    pub struct SymbolIndex {}
}");
            }
        }
        path if path.contains("hygiene.rs") => {
            if !result.contains("pub type LocalExpnId") {
                result = add_after_imports(&result, "
// Add missing types
rustc_index::newtype_index! {
    pub struct LocalExpnId {}
}

rustc_index::newtype_index! {
    pub struct ExpnIndex {}
}");
            }
        }
        path if path.contains("lib.rs") && path.contains("rustc_span") => {
            if !result.contains("pub type AttrId") {
                result = add_after_doc_comments(&result, "
// Add missing types
pub type AttrId = u32;");
            }
        }
        _ => {}
    }
    
    result
}

/// Fix jobserver imports to use correct crate alias
pub fn fix_jobserver_imports(content: &str) -> String {
    content
        .replace("pub use jobserver::", "pub use jobserver_crate::")
        .replace("use jobserver::", "use jobserver_crate::")
        .replace("[jobserver::", "[crate::jobserver::")
}

/// Add required imports based on compiler suggestions
pub fn add_required_imports(content: &str, file_path: &str) -> String {
    let mut result = content.to_string();
    
    // Add FiniteBitSetTy import where needed
    if result.contains("FiniteBitSetTy") && !result.contains("use rustc_index::bit_set::FiniteBitSetTy") {
        result = add_after_std_imports(&result, "use rustc_index::bit_set::FiniteBitSetTy;");
    }
    
    result
}

/// Create stub implementations for problematic crates
pub fn create_stub_implementation(crate_name: &str) -> Option<String> {
    match crate_name {
        "rustc_proc_macro" => Some(r#"// Stub implementation of rustc_proc_macro to avoid standard library proc_macro issues
#![allow(unused)]

pub struct TokenStream;
pub struct Group;
pub struct Ident;
pub struct Punct;
pub struct Literal;
pub struct Span;

impl TokenStream {
    pub fn new() -> Self { TokenStream }
}

pub mod bridge {
    pub struct Client;
    pub struct Server;
}

pub mod diagnostic {
    pub struct Diagnostic;
}"#.to_string()),
        
        "rustc_baked_icu_data" => Some(r#"// Stub data file for rustc_baked_icu_data
macro_rules! impl_list_and_v1 {
    ($provider:ty) => {
        // Stub implementation
    };
}

pub const DATA: &[u8] = &[];"#.to_string()),
        
        _ => None
    }
}

// Helper functions

fn add_after_imports(content: &str, addition: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut insert_pos = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("use ") {
            insert_pos = i + 1;
        }
    }
    
    let mut new_lines = lines;
    new_lines.insert(insert_pos, Box::leak(addition.into_boxed_str()));
    new_lines.join("\n")
}

fn add_after_std_imports(content: &str, addition: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut insert_pos = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("use std::") {
            insert_pos = i + 1;
        }
    }
    
    let mut new_lines = lines;
    new_lines.insert(insert_pos, Box::leak(addition.into_boxed_str()));
    new_lines.join("\n")
}

fn add_after_doc_comments(content: &str, addition: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut insert_pos = 0;
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && 
           !trimmed.starts_with("//!") && 
           !trimmed.starts_with("#!") {
            insert_pos = i;
            break;
        }
    }
    
    let mut new_lines = lines;
    new_lines.insert(insert_pos, Box::leak(addition.into_boxed_str()));
    new_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_bootstrap_features() {
        let input = "#![feature(existing)]\nuse std::fmt;";
        let result = add_bootstrap_features(input);
        assert!(result.contains("#![feature(rustc_private)]"));
        assert!(result.contains("#![feature(existing)]"));
    }

    #[test]
    fn test_strip_malformed_doc_attributes() {
        let input = r#"#![doc(
    html_root_url = "test",
    test(attr(deny(warnings)))
)]
use std::fmt;"#;
        let result = strip_malformed_doc_attributes(input);
        assert_eq!(result.trim(), "use std::fmt;");
    }

    #[test]
    fn test_fix_jobserver_imports() {
        let input = "pub use jobserver::{Client, Server};";
        let result = fix_jobserver_imports(input);
        assert_eq!(result, "pub use jobserver_crate::{Client, Server};");
    }
}
