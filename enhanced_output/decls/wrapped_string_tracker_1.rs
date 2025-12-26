// Generated from: ./src/string_tracker.rs
// Original file: ./src/string_tracker.rs
// Function: contains_ignoring_case

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

#[decl_split_decls_rs_string_tracker]
# [doc = " Case-insensitive substring search"] pub fn contains_ignoring_case (haystack : & str , needle : & str) -> bool { haystack . to_lowercase () . contains (& needle . to_lowercase ()) }