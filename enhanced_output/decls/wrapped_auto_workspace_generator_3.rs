// Generated from: ./src/auto_workspace_generator.rs
// Original file: ./src/auto_workspace_generator.rs
// Function: extract_crate_info

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

#[decl_split_decls_rs_auto_workspace_generator]
fn extract_crate_info (cargo_path : & Path) -> Result < Option < CrateInfo > > { let content = fs :: read_to_string (cargo_path) ? ; let toml : Value = toml :: from_str (& content) ? ; if let Some (package) = toml . get ("package") { if let Some (name) = package . get ("name") . and_then (| n | n . as_str ()) { return Ok (Some (CrateInfo { name : name . to_string () , })) ; } } Ok (None) }