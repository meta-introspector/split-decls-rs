// Generated from: ./src/bin/decl_histogram.rs
// Original file: ./src/bin/decl_histogram.rs
// Function: is_likely_enum

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

#[decl_split_decls_rs_decl_histogram]
fn is_likely_enum (name : & str) -> bool { name . chars () . next () . map_or (false , | c | c . is_uppercase ()) && name . contains ("Kind") }