// Generated from: ./src/bin/sync_workspace_members.rs
// Original file: ./src/bin/sync_workspace_members.rs
// Function: find_local_workspace_crates

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

#[decl_split_decls_rs_sync_workspace_members]
fn find_local_workspace_crates (dir : & Path) -> Result < Vec < String > > { let mut crates = Vec :: new () ; for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { let cargo_toml = path . join ("Cargo.toml") ; if cargo_toml . exists () { if let Some (name) = path . file_name () { crates . push (name . to_string_lossy () . to_string ()) ; } } } } crates . sort () ; Ok (crates) }