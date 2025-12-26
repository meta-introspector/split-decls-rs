// Generated from: ./src/bin/recursive_compressor.rs
// Original file: ./src/bin/recursive_compressor.rs
// Function: apply_compression

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

#[decl_split_decls_rs_recursive_compressor]
fn apply_compression (text : & str , vocab : & HashMap < String , String >) -> String { let mut result = text . to_string () ; let mut sorted_patterns : Vec < _ > = vocab . iter () . collect () ; sorted_patterns . sort_by (| a , b | b . 1 . len () . cmp (& a . 1 . len ())) ; for (emoji , pattern) in sorted_patterns { result = result . replace (pattern , emoji) ; } result }