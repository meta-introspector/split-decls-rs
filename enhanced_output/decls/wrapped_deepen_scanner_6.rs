// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: extract_macro_content

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

#[decl_split_decls_rs_deepen_scanner]
fn extract_macro_content (line : & str) -> Option < String > { if let Some (content_start) = line . rfind ('"') { if let Some (content_end) = line [.. content_start] . rfind ('"') { return Some (line [content_end + 1 .. content_start] . to_string ()) ; } } None }