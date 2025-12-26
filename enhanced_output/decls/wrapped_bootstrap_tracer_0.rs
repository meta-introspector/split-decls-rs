// Generated from: ./src/bootstrap_tracer.rs
// Original file: ./src/bootstrap_tracer.rs
// Function: sanitize_lean_id

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

#[decl_split_decls_rs_bootstrap_tracer]
fn sanitize_lean_id (id : & str) -> String { id . chars () . map (| c | if c . is_alphanumeric () || c == '_' { c } else { '_' }) . collect :: < String > () . trim_start_matches (| c : char | c . is_numeric ()) . to_string () }