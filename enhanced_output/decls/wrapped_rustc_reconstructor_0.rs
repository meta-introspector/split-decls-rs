// Generated from: ./src/bin/rustc_reconstructor.rs
// Original file: ./src/bin/rustc_reconstructor.rs
// Function: reverse_layer

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

#[decl_split_decls_rs_rustc_reconstructor]
fn reverse_layer (vocab : & HashMap < String , String > , input_tokens : & [String]) -> Vec < String > { let mut expanded = Vec :: new () ; for token in input_tokens { if let Some (pattern) = vocab . get (token) { expanded . push (pattern . clone ()) ; } else { expanded . push (token . clone ()) ; } } expanded }