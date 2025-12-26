// Generated from: ./src/bin/add_wrapped_crate.rs
// Original file: ./src/bin/add_wrapped_crate.rs
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

#[decl_split_decls_rs_add_wrapped_crate]
fn main () -> Result < () > { let args = Args :: parse () ; println ! ("Generating wrapped crate for: {}" , args . crate_path) ; let output = Command :: new ("make") . arg ("run_single_crate") . arg (& format ! ("CRATE={}" , args . crate_path)) . output () ? ; if ! output . status . success () { eprintln ! ("Failed to wrap crate: {}" , String :: from_utf8_lossy (& output . stderr)) ; return Err (anyhow :: anyhow ! ("Single crate wrapper failed")) ; } if args . verbose { println ! ("Wrapper output: {}" , String :: from_utf8_lossy (& output . stdout)) ; } let crate_name = Path :: new (& args . crate_path) . file_name () . unwrap () . to_string_lossy () ; let wrapped_name = format ! ("wrapped-{}" , crate_name) ; add_to_root_workspace (& wrapped_name) ? ; println ! ("✅ Successfully added {} to root workspace" , wrapped_name) ; Ok (()) }