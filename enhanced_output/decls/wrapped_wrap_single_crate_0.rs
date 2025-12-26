// Generated from: ./src/bin/wrap_single_crate.rs
// Original file: ./src/bin/wrap_single_crate.rs
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

#[decl_split_decls_rs_wrap_single_crate]
fn main () -> Result < () > { let args = Args :: parse () ; let output_dir = args . output . unwrap_or_else (| | PathBuf :: from ("output2")) ; let cargo_toml_path = args . crate_path . join ("Cargo.toml") ; let cargo_content = std :: fs :: read_to_string (& cargo_toml_path) . context ("Failed to read Cargo.toml") ? ; let cargo_toml : toml :: Value = toml :: from_str (& cargo_content) . context ("Failed to parse Cargo.toml") ? ; let crate_name = cargo_toml ["package"] ["name"] . as_str () . context ("Failed to extract crate name from Cargo.toml") ? . to_string () ; if args . verbose { println ! ("Processing crate: {} at {}" , crate_name , args . crate_path . display ()) ; println ! ("Output directory: {}" , output_dir . display ()) ; } let global_config = SplitDeclsConfig :: default () ; let patch_config = PatchConfig :: default () ; std :: fs :: create_dir_all (& output_dir) . context ("Failed to create output directory") ? ; generate_wrapped_crate (& output_dir , & crate_name , & args . crate_path , & global_config , & patch_config , false , false ,) . context (format ! ("Failed to wrap crate '{}'" , crate_name)) ? ; println ! ("✅ Successfully wrapped crate '{}' to '{}'" , crate_name , output_dir . display ()) ; Ok (()) }