// Generated from: ./src/all_file_scanner.rs
// Original file: ./src/all_file_scanner.rs
// Function: scan_all_rust_files

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

#[decl_split_decls_rs_all_file_scanner]
# [doc = " Scan all Rust source files in a crate and extract functions"] pub fn scan_all_rust_files (crate_path : & Path) -> Result < Vec < (PathBuf , Vec < Item >) > > { let mut all_files = Vec :: new () ; let src_dir = crate_path . join ("src") ; if src_dir . exists () { scan_rust_files_recursive (& src_dir , & mut all_files) ? ; } Ok (all_files) }