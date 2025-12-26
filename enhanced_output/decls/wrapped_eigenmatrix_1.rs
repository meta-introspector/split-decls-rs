// Generated from: ./src/bin/eigenmatrix.rs
// Original file: ./src/bin/eigenmatrix.rs
// Function: analyze_directory

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
fn analyze_directory (path : & Path , crates : & mut Vec < CrateMetrics >) -> Result < () > { if ! path . exists () { return Err (anyhow :: anyhow ! ("Path does not exist: {:?}" , path)) ; } for entry in std :: fs :: read_dir (path) ? { let entry = entry ? ; let entry_path = entry . path () ; if entry_path . is_dir () { let cargo_toml = entry_path . join ("Cargo.toml") ; if cargo_toml . exists () { let crate_name = entry_path . file_name () . unwrap_or_default () . to_string_lossy () . to_string () ; let metrics = analyze_crate (& entry_path , crate_name) ? ; crates . push (metrics) ; } analyze_directory (& entry_path , crates) ? ; } } Ok (()) }