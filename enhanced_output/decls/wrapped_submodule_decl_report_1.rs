// Generated from: ./src/bin/submodule_decl_report.rs
// Original file: ./src/bin/submodule_decl_report.rs
// Function: extract_submodule_name

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

#[decl_split_decls_rs_submodule_decl_report]
fn extract_submodule_name (decls_path : & Path) -> Option < String > { decls_path . parent () ? . parent () ? . file_name () ? . to_str () . map (| s | s . to_string ()) }