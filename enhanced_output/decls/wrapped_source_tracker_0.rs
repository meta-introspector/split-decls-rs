// Generated from: ./src/source_tracker.rs
// Original file: ./src/source_tracker.rs
// Function: span_to_location

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

#[decl_split_decls_rs_source_tracker]
# [doc = " Extract source location from a span"] pub fn span_to_location (span : Span) -> SourceLocation { let line_col : LineColumn = span . start () ; SourceLocation { file : "unknown" . to_string () , line : line_col . line , column : line_col . column , } }