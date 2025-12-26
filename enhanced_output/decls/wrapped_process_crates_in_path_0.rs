// Generated from: ./src/process_crates_in_path.rs
// Original file: ./src/process_crates_in_path.rs
// Function: process_crates_in_path

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

#[decl_split_decls_rs_process_crates_in_path]
# [doc = " Process all crates in a given path, applying the split-decls transformation."] pub fn process_crates_in_path (root_path : & Path , current_crate_name : & str , global_config : & SplitDeclsConfig , _is_rustc_source : bool , dry_run : bool ,) -> Result < () > { if dry_run { println ! ("DRY RUN: Would process crates in {}" , root_path . display ()) ; return Ok (()) ; } println ! ("Processing crates in path: {}" , root_path . display ()) ; let cargo_toml_path = root_path . join ("Cargo.toml") ; if cargo_toml_path . exists () { let crate_name = root_path . file_name () . and_then (| n | n . to_str ()) . unwrap_or ("unknown") ; if crate_name != current_crate_name { process_crate (root_path , global_config , dry_run) ? ; } } Ok (()) }