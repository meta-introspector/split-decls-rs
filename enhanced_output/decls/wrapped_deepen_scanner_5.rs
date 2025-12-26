// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: extract_macro_name

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
fn extract_macro_name (line : & str) -> Option < String > { if let Some (start) = line . find ("sys:macro") { if let Some (name_start) = line [start ..] . find ('"') { if let Some (name_end) = line [start + name_start + 1 ..] . find ('"') { return Some (line [start + name_start + 1 .. start + name_start + 1 + name_end] . to_string ()) ; } } } None }