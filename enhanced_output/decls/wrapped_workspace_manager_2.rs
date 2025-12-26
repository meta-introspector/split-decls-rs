// Generated from: ./src/workspace_manager.rs
// Original file: ./src/workspace_manager.rs
// Function: get_default_workspace_package

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

#[decl_split_decls_rs_workspace_manager]
fn get_default_workspace_package () -> toml :: Table { let mut default_package = toml :: Table :: new () ; default_package . insert ("edition" . to_string () , toml :: Value :: String ("2024" . to_string ())) ; default_package . insert ("version" . to_string () , toml :: Value :: String ("1.0.0" . to_string ())) ; default_package . insert ("publish" . to_string () , toml :: Value :: Boolean (false)) ; default_package . insert ("keywords" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("rust-version" . to_string () , toml :: Value :: String ("1.85.0" . to_string ())) ; default_package . insert ("include" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("license" . to_string () , toml :: Value :: String ("AGPL 3.0" . to_string ())) ; default_package . insert ("authors" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("description" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package . insert ("categories" . to_string () , toml :: Value :: Array (vec ! [])) ; default_package . insert ("repository" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package . insert ("homepage" . to_string () , toml :: Value :: String ("" . to_string ())) ; default_package }