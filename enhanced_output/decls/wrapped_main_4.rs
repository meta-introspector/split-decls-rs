// Generated from: ./src/main.rs
// Original file: ./src/main.rs
// Function: dep_to_toml_value_iter

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

#[decl_split_decls_rs_main]
# [doc = " Helper function to convert an iterator of (String, cargo_toml_generator_types::Dependency)"] # [doc = " to an iterator of (String, toml::Value)."] fn dep_to_toml_value_iter < 'a > (iter : impl IntoIterator < Item = (String , Dependency) > + 'a ,) -> impl Iterator < Item = (String , toml :: Value) > + 'a { iter . into_iter () . filter_map (| (name , dep) | { match toml :: to_string (& dep) { Ok (serialized_dep) => { match toml :: from_str (& serialized_dep) { Ok (toml_value) => Some ((name , toml_value)) , Err (_) => { eprintln ! ("Warning: Failed to parse serialized Dependency TOML for {}" , name) ; None } } } Err (_) => { eprintln ! ("Warning: Failed to serialize Dependency to TOML for {}" , name) ; None } } }) }