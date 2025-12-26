// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: calculate_similarity

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

#[decl_split_decls_rs_deepen_scanner]
fn calculate_similarity (content1 : & str , content2 : & str) -> f64 { let tokens1 : Vec < & str > = content1 . split_whitespace () . collect () ; let tokens2 : Vec < & str > = content2 . split_whitespace () . collect () ; let rust_patterns = ["HashMap" , "Result" , "anyhow" , "fs::" , "std::" , "use" , "fn" , "struct" , "impl"] ; let pattern_matches = rust_patterns . iter () . filter (| pattern | content1 . contains (* pattern) && content2 . contains (* pattern)) . count () ; let common_tokens = tokens1 . iter () . filter (| token | tokens2 . contains (token)) . count () ; let total_tokens = (tokens1 . len () + tokens2 . len ()) as f64 ; let token_similarity = if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens } ; let pattern_boost = (pattern_matches as f64) * 0.1 ; (token_similarity + pattern_boost) . min (1.0) }