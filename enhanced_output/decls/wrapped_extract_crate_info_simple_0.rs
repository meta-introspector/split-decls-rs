// Generated from: ./src/extract_crate_info_simple.rs
// Original file: ./src/extract_crate_info_simple.rs
// Function: extract_crate_info_simple

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

#[decl_split_decls_rs_extract_crate_info_simple]
pub fn extract_crate_info_simple (cargo_path : & Path) -> Result < Option < SimpleCrateInfo > > { let cargo_toml_content = fs :: read_to_string (cargo_path) . context (format ! ("Failed to read Cargo.toml at {}" , cargo_path . display ())) ? ; let cargo_toml : Table = toml :: from_str (& cargo_toml_content) . context (format ! ("Failed to parse Cargo.toml at {}" , cargo_path . display ())) ? ; if let Some (package_table) = cargo_toml . get ("package") . and_then (| v | v . as_table ()) { let name = package_table . get ("name") . and_then (| v | v . as_str ()) . map (| s | s . to_string ()) ; let version = package_table . get ("version") . and_then (| v | v . as_str ()) . map (| s | s . to_string ()) ; if let (Some (name) , Some (version)) = (name , version) { return Ok (Some (SimpleCrateInfo { name , version , manifest_path : cargo_path . to_path_buf () , })) ; } } Ok (None) }