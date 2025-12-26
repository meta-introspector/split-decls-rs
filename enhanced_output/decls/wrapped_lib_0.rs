// Generated from: ./src/lib.rs
// Original file: ./src/lib.rs
// Function: process_dependency_table

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

#[decl_split_decls_rs_lib]
fn process_dependency_table (table : & mut toml :: Table , global_config : & SplitDeclsConfig , original_crate_path : & Path ,) -> Result < () > { let mut deps_to_update_to_workspace = Vec :: new () ; for (dep_name , dep_value) in table . iter () { deps_to_update_to_workspace . push (dep_name . clone ()) ; } for dep_name in deps_to_update_to_workspace { let mut new_dep_table = toml :: Table :: new () ; new_dep_table . insert ("workspace" . to_string () , toml :: Value :: Boolean (true)) ; if let Some (original_dep) = table . get (& dep_name) { if let Some (original_dep_table) = original_dep . as_table () { if let Some (features) = original_dep_table . get ("features") { new_dep_table . insert ("features" . to_string () , features . clone ()) ; } if let Some (optional) = original_dep_table . get ("optional") { new_dep_table . insert ("optional" . to_string () , optional . clone ()) ; } if let Some (default_features) = original_dep_table . get ("default-features") { new_dep_table . insert ("default-features" . to_string () , default_features . clone ()) ; } } } table . insert (dep_name , toml :: Value :: Table (new_dep_table)) ; } Ok (()) }