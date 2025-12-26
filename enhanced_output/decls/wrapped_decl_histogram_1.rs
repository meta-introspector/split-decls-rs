// Generated from: ./src/bin/decl_histogram.rs
// Original file: ./src/bin/decl_histogram.rs
// Function: find_decl_files

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

#[decl_split_decls_rs_decl_histogram]
fn find_decl_files (path : & PathBuf , histogram : & mut HashMap < String , u32 > , total : & mut u32) -> Result < () , Box < dyn std :: error :: Error > > { if let Ok (entries) = fs :: read_dir (path) { for entry in entries { if let Ok (entry) = entry { let entry_path = entry . path () ; if entry_path . is_dir () { let name = entry_path . file_name () . unwrap () . to_string_lossy () ; if ! name . starts_with ('.') && name != "target" { find_decl_files (& entry_path , histogram , total) ? ; } } else if let Some (filename) = entry_path . file_name () { let filename_str = filename . to_string_lossy () ; if filename_str . contains ("_decls_") && filename_str . ends_with (".rs") { * total += 1 ; if let Some (decl_type) = extract_decl_type (& filename_str) { * histogram . entry (decl_type) . or_insert (0) += 1 ; } } } } } } Ok (()) }