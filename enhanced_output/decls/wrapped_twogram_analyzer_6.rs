// Generated from: ./src/bin/twogram_analyzer.rs
// Original file: ./src/bin/twogram_analyzer.rs
// Function: create_compression_mapping

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
fn create_compression_mapping (input_patterns : & [serde_json :: Value] , output_emojis : & [serde_json :: Value]) -> HashMap < String , String > { let mut mapping = HashMap :: new () ; for (i , pattern) in input_patterns . iter () . enumerate () { let pattern_str = pattern . as_str () . unwrap_or ("") ; let emoji_idx = i % output_emojis . len () ; let emoji = output_emojis [emoji_idx] . as_str () . unwrap_or ("") ; if ! pattern_str . is_empty () && ! emoji . is_empty () { mapping . insert (pattern_str . to_string () , emoji . to_string ()) ; } } mapping }