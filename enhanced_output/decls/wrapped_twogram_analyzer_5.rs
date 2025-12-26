// Generated from: ./src/bin/twogram_analyzer.rs
// Original file: ./src/bin/twogram_analyzer.rs
// Function: check_preservation

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

#[decl_split_decls_rs_twogram_analyzer]
fn check_preservation (tokens : & (String , String) , input_2grams : & HashMap < (String , String) , usize > , output_2grams : & HashMap < (String , String) , usize >) -> bool { let input_count = input_2grams . get (tokens) . unwrap_or (& 0) ; for (output_pair , output_count) in output_2grams { if output_count >= input_count { return true ; } } * input_count > 5 }