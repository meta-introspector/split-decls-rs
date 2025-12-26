// Generated from: ./src/bin/layer_common_forms.rs
// Original file: ./src/bin/layer_common_forms.rs
// Function: analyze_common_forms

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

#[decl_split_decls_rs_layer_common_forms]
fn analyze_common_forms (patterns : & [serde_json :: Value] , label : & str) -> Vec < (String , usize) > { let mut pattern_counts = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let common_form = extract_common_form (pattern_str) ; * pattern_counts . entry (common_form) . or_insert (0) += 1 ; } let mut sorted_patterns : Vec < _ > = pattern_counts . into_iter () . collect () ; sorted_patterns . sort_by (| a , b | b . 1 . cmp (& a . 1)) ; sorted_patterns }