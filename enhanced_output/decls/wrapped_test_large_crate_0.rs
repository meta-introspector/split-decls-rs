// Generated from: ./src/bin/test_large_crate.rs
// Original file: ./src/bin/test_large_crate.rs
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

#[decl_split_decls_rs_test_large_crate]
fn main () -> anyhow :: Result < () > { let crate_path = PathBuf :: from ("../../crates/rustc_ast_fragments") ; let paths = setup_crate_paths (& crate_path) ? ; let config = SplitDeclsConfig :: default () ; println ! ("Processing largest crate: rustc_ast_fragments") ; println ! ("Input: {}" , crate_path . display ()) ; println ! ("Output: {}" , paths . decls_output_dir . display ()) ; eager_splitter :: eager_split_crate (& paths , & config) ? ; let decl_count = std :: fs :: read_dir (& paths . decls_output_dir) ? . count () ; println ! ("✅ Generated {} declaration files!" , decl_count) ; Ok (()) }