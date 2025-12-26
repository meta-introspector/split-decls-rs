// Generated from: ./src/bin/twogram_analyzer.rs
// Original file: ./src/bin/twogram_analyzer.rs
// Function: extract_2grams_from_patterns

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
fn extract_2grams_from_patterns (patterns : & [serde_json :: Value]) -> HashMap < (String , String) , usize > { let mut twograms = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let tokens = tokenize_pattern (pattern_str) ; for window in tokens . windows (2) { if window . len () == 2 { let pair = (window [0] . clone () , window [1] . clone ()) ; * twograms . entry (pair) . or_insert (0) += 1 ; } } } twograms }