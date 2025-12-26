// Generated from: ./src/bin/test_auto_workspace.rs
// Original file: ./src/bin/test_auto_workspace.rs
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

#[decl_split_decls_rs_test_auto_workspace]
fn main () -> anyhow :: Result < () > { let project_root = Path :: new ("../../") ; match split_decls_rs :: auto_workspace_generator :: generate_workspace_deps_from_project_root (& project_root) { Ok ((deps , patches)) => { println ! ("Found {} workspace deps and {} patches" , deps . len () , patches . len ()) ; for (i , dep) in deps . iter () . enumerate () . take (5) { println ! ("Dep {}: {}" , i + 1 , dep) ; } } Err (e) => { println ! ("Error: {}" , e) ; } } Ok (()) }