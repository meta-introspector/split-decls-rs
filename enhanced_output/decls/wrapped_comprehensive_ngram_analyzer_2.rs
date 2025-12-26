// Generated from: ./src/bin/comprehensive_ngram_analyzer.rs
// Original file: ./src/bin/comprehensive_ngram_analyzer.rs
// Function: extract_top_ngrams

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

#[decl_split_decls_rs_comprehensive_ngram_analyzer]
fn extract_top_ngrams (patterns : & [serde_json :: Value] , n : usize) -> Vec < NGram > { let mut ngram_counts = HashMap :: new () ; for pattern in patterns { let pattern_str = pattern . as_str () . unwrap_or ("") ; let tokens = tokenize_pattern (pattern_str) ; for window in tokens . windows (n) { if window . len () == n { let ngram = window . to_vec () ; * ngram_counts . entry (ngram) . or_insert (0) += 1 ; } } } let mut sorted_ngrams : Vec < _ > = ngram_counts . iter () . collect () ; sorted_ngrams . sort_by (| a , b | b . 1 . cmp (a . 1)) ; sorted_ngrams . iter () . take (10) . map (| (tokens , count) | NGram { tokens : tokens . to_vec () , count : * * count , pattern_type : classify_ngram_pattern (tokens , n) , }) . collect () }