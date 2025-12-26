// Generated from: ./src/bin/comprehensive_ngram_analyzer.rs
// Original file: ./src/bin/comprehensive_ngram_analyzer.rs
// Function: tokenize_pattern

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
fn tokenize_pattern (pattern : & str) -> Vec < String > { pattern . split (['/' , '_' , '.' , '-' , ':' , ' ']) . filter (| s | ! s . is_empty () && s . len () > 0) . map (| s | s . to_lowercase ()) . collect () }