// Generated from: ./src/bin/add_wrapped_crate.rs
// Original file: ./src/bin/add_wrapped_crate.rs
// Function: add_to_root_workspace

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

#[decl_split_decls_rs_add_wrapped_crate]
fn add_to_root_workspace (wrapped_name : & str) -> Result < () > { use std :: fs ; use toml_edit :: { DocumentMut , value } ; let root_cargo_path = "../../Cargo.toml" ; let content = fs :: read_to_string (root_cargo_path) ? ; let mut doc = content . parse :: < DocumentMut > () ? ; let members_array = doc . get_mut ("workspace") . and_then (| w | w . get_mut ("members")) . and_then (| m | m . as_array_mut ()) . ok_or_else (| | anyhow :: anyhow ! ("No workspace.members found")) ? ; let member_path = format ! ("submodules/split-decls-rs/output2/{}" , wrapped_name) ; let exists = members_array . iter () . any (| item | { item . as_str () == Some (& member_path) }) ; if ! exists { members_array . push (& member_path) ; println ! ("Added {} to workspace members" , member_path) ; fs :: write (root_cargo_path , doc . to_string ()) ? ; } else { println ! ("Member {} already exists" , member_path) ; } Ok (()) }