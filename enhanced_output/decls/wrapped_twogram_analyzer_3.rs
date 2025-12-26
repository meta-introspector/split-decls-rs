// Generated from: ./src/bin/twogram_analyzer.rs
// Original file: ./src/bin/twogram_analyzer.rs
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

#[decl_split_decls_rs_twogram_analyzer]
fn tokenize_pattern (pattern : & str) -> Vec < String > { pattern . split (['/' , '_' , '.' , '-' , ':']) . filter (| s | ! s . is_empty () && s . len () > 1) . map (| s | s . to_lowercase ()) . collect () }