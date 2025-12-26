// Generated from: ./src/process_crate.rs
// Original file: ./src/process_crate.rs
// Function: process_crate

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

#[decl_split_decls_rs_process_crate]
# [doc = " Process a single crate by applying the split-decls transformation."] pub fn process_crate (crate_path : & Path , global_config : & SplitDeclsConfig , dry_run : bool) -> Result < () > { let paths = setup_crate_paths (crate_path) ? ; if dry_run { println ! ("DRY RUN: Would process crate at {}" , crate_path . display ()) ; return Ok (()) ; } println ! ("Processing crate: {}" , paths . crate_name) ; if ! paths . lib_rs_path . exists () { println ! ("No lib.rs found at {}, skipping" , paths . lib_rs_path . display ()) ; return Ok (()) ; } eager_split_crate (& paths , global_config) ? ; println ! ("Successfully processed crate: {}" , paths . crate_name) ; Ok (()) }