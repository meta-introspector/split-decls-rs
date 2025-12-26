// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: semantic_match_score

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

#[decl_split_decls_rs_dwim_macros]
fn semantic_match_score (intent : & DwimIntent , macro_def : & MacroDefinition) -> f64 { if intent . keywords . contains (& "bootstrap" . to_string ()) && macro_def . name . contains ("bootstrap") { 0.9 } else if intent . keywords . contains (& "build" . to_string ()) && macro_def . name . contains ("build") { 0.8 } else { 0.0 } }