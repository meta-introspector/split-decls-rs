// Generated from: ./src/find_all_cargo_tomls.rs
// Original file: ./src/find_all_cargo_tomls.rs
// Function: find_all_cargo_tomls

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

#[decl_split_decls_rs_find_all_cargo_tomls]
pub fn find_all_cargo_tomls (dir : & Path , verbose : bool) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if verbose { println ! ("DEBUG: find_all_cargo_tomls: Scanning directory: {}" , dir . display ()) ; } for entry in WalkDir :: new (dir) . into_iter () . filter_map (| e | e . ok ()) { let path = entry . path () ; if path . is_file () && path . file_name () . map_or (false , | name | name == "Cargo.toml") { if verbose { println ! ("DEBUG: Found Cargo.toml: {}" , path . display ()) ; } cargo_tomls . push (path . to_path_buf ()) ; } } if verbose { println ! ("DEBUG: Finished scanning. Found {} Cargo.toml files." , cargo_tomls . len ()) ; } Ok (cargo_tomls) }