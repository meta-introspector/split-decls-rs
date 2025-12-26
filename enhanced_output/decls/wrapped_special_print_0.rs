// Generated from: ./src/special_print.rs
// Original file: ./src/special_print.rs
// Function: specialprint

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

#[decl_split_decls_rs_special_print]
pub fn specialprint (macro_name : & str , local_score : f64) { print ! (", {}: {:.4}" , macro_name , local_score) ; }