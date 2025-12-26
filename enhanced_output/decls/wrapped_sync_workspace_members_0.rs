// Generated from: ./src/bin/sync_workspace_members.rs
// Original file: ./src/bin/sync_workspace_members.rs
// Function: main

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
fn main () -> Result < () > { let parent_cargo_path = Path :: new ("../../Cargo.toml") ; let current_dir = Path :: new (".") ; println ! ("Syncing workspace members to parent Cargo.toml...") ; let local_crates = find_local_workspace_crates (current_dir) ? ; update_parent_workspace (& parent_cargo_path , & local_crates) ? ; println ! ("Successfully synced {} workspace members" , local_crates . len ()) ; Ok (()) }