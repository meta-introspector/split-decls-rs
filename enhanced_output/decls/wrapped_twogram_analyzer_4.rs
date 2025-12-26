// Generated from: ./src/bin/twogram_analyzer.rs
// Original file: ./src/bin/twogram_analyzer.rs
// Function: classify_relationship

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use split_decls_types::SplitDeclsConfig;
pub use extracted_decl::*;
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;
prelude!{}

#[decl_split_decls_rs_twogram_analyzer]
fn classify_relationship (tokens : & (String , String)) -> String { let (first , second) = tokens ; match (first . as_str () , second . as_str ()) { (a , b) if a . contains ("src") && b . contains ("lib") => "source_library" . to_string () , (a , b) if a . contains ("cargo") && b . contains ("toml") => "config_file" . to_string () , (a , b) if a . contains ("decls") => "declaration_module" . to_string () , (a , b) if a . ends_with ("rs") => "rust_source" . to_string () , (a , b) if a . contains ("bin") => "binary_executable" . to_string () , (a , b) if a . contains ("test") => "test_module" . to_string () , _ => "generic_relationship" . to_string () , } }