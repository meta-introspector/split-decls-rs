// Generated from: ./src/bin/test_workspace_crate.rs
// Original file: ./src/bin/test_workspace_crate.rs
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

#[decl_split_decls_rs_test_workspace_crate]
fn main () -> anyhow :: Result < () > { println ! ("Testing direct eager splitter on a workspace crate...") ; let crate_path = PathBuf :: from ("../../crates/monster_traits") ; if ! crate_path . join ("src/lib.rs") . exists () { println ! ("Crate doesn't have src/lib.rs, skipping...") ; return Ok (()) ; } println ! ("Processing crate: {}" , crate_path . display ()) ; let paths = setup_crate_paths (& crate_path) ? ; let config = SplitDeclsConfig :: default () ; println ! ("Output will be generated to: {}" , paths . decls_output_dir . display ()) ; eager_splitter :: eager_split_crate (& paths , & config) ? ; println ! ("✅ Successfully processed crate!") ; println ! ("Check the output directory: {}" , paths . decls_output_dir . display ()) ; Ok (()) }