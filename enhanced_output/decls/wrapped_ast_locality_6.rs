// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
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

#[decl_split_decls_rs_ast_locality]
fn calculate_similarity (content1 : & str , content2 : & str) -> f64 { let tokens1 : Vec < & str > = content1 . split_whitespace () . collect () ; let tokens2 : Vec < & str > = content2 . split_whitespace () . collect () ; let common_tokens = tokens1 . par_iter () . filter (| token | tokens2 . contains (token)) . count () ; let total_tokens = (tokens1 . len () + tokens2 . len ()) as f64 ; if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens } }