// Generated from: ./src/bin/sync_workspace_members.rs
// Original file: ./src/bin/sync_workspace_members.rs
// Function: update_parent_workspace

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
fn update_parent_workspace (cargo_path : & Path , new_members : & [String]) -> Result < () > { let content = fs :: read_to_string (cargo_path) ? ; let mut doc = content . parse :: < toml_edit :: DocumentMut > () ? ; if ! doc . contains_key ("workspace") { doc ["workspace"] = toml_edit :: table () ; } if ! doc ["workspace"] . as_table () . unwrap () . contains_key ("members") { doc ["workspace"] ["members"] = value (Array :: new ()) ; } let members_array = doc ["workspace"] ["members"] . as_array_mut () . unwrap () ; for member in new_members { let full_path = format ! ("submodules/split-decls-rs/{}" , member) ; let exists = members_array . iter () . any (| item | { item . as_str () == Some (& full_path) }) ; if ! exists { members_array . push (full_path) ; println ! ("Added workspace member: submodules/split-decls-rs/{}" , member) ; } } fs :: write (cargo_path , doc . to_string ()) ? ; Ok (()) }