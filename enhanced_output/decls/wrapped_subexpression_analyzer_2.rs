// Generated from: ./src/bin/subexpression_analyzer.rs
// Original file: ./src/bin/subexpression_analyzer.rs
// Function: find_rust_files

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

#[decl_split_decls_rs_subexpression_analyzer]
fn find_rust_files (dir : & Path) -> Vec < std :: path :: PathBuf > { let mut rust_files = Vec :: new () ; if let Ok (entries) = fs :: read_dir (dir) { for entry in entries . flatten () { let path = entry . path () ; if path . is_dir () && ! path . file_name () . unwrap_or_default () . to_string_lossy () . starts_with ('.') { rust_files . extend (find_rust_files (& path)) ; } else if path . extension () . map_or (false , | ext | ext == "rs") { rust_files . push (path) ; } } } rust_files }