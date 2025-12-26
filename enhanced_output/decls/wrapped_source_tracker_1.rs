// Generated from: ./src/source_tracker.rs
// Original file: ./src/source_tracker.rs
// Function: create_source_map

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
# [doc = " Create a source map for a token stream"] # [doc = " This is a simplified version - a full implementation would need to"] # [doc = " track the expansion of each token"] pub fn create_source_map (tokens : & TokenStream) -> SourceMap { let mut source_map = SourceMap :: new () ; source_map }