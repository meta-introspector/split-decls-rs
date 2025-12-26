// Generated from: ./src/bin/analyze_common_terms.rs
// Original file: ./src/bin/analyze_common_terms.rs
// Function: split_camel_case

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

#[decl_split_decls_rs_analyze_common_terms]
# [doc = " Split camelCase into separate terms"] fn split_camel_case (s : & str) -> Vec < String > { let mut terms = Vec :: new () ; let mut current = String :: new () ; for c in s . chars () { if c . is_uppercase () && ! current . is_empty () { terms . push (current . clone ()) ; current . clear () ; } current . push (c) ; } if ! current . is_empty () { terms . push (current) ; } terms }