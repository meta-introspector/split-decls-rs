// Generated from: ./src/bin/analyze_common_terms.rs
// Original file: ./src/bin/analyze_common_terms.rs
// Function: extract_terms

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
# [doc = " Extract terms from a declaration name"] fn extract_terms (name : & str) -> Vec < String > { let mut terms = Vec :: new () ; for part in name . split ('_') { if ! part . is_empty () { terms . push (part . to_lowercase ()) ; let camel_terms = split_camel_case (part) ; for term in camel_terms { if term . len () >= 2 { terms . push (term . to_lowercase ()) ; } } } } terms . sort () ; terms . dedup () ; terms }