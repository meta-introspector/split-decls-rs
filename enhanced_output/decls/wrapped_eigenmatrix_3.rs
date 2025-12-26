// Generated from: ./src/bin/eigenmatrix.rs
// Original file: ./src/bin/eigenmatrix.rs
// Function: analyze_rust_files

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

#[decl_split_decls_rs_eigenmatrix]
fn analyze_rust_files (dir : & Path , metrics : & mut CrateMetrics) -> Result < () > { for entry in std :: fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { let content = std :: fs :: read_to_string (& path) ? ; analyze_rust_content (& content , metrics) ; } else if path . is_dir () { analyze_rust_files (& path , metrics) ? ; } } Ok (()) }