// Generated from: ./src/bin/rust_eigenmatrix.rs
// Original file: ./src/bin/rust_eigenmatrix.rs
// Function: load_cached_matrix

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

#[decl_split_decls_rs_rust_eigenmatrix]
fn load_cached_matrix (cache_file : & str) -> Result < CachedMatrix > { let content = fs :: read_to_string (cache_file) ? ; let cached : CachedMatrix = serde_json :: from_str (& content) ? ; Ok (cached) }