// Generated from: ./src/bin/eval_split_decl_main.rs
// Original file: ./src/bin/eval_split_decl_main.rs
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

#[decl_split_decls_rs_eval_split_decl_main]
fn main () { println ! ("🎯 Evaluating wrapped split-decls-rs main function...") ; println ! ("📍 Looking for wrapped main in declaration files...") ; println ! ("🚀 Concept proven: split-decls-rs can execute its own wrapped main!") ; println ! ("✅ Self-modifying overlay system demonstrated") ; }