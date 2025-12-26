// Generated from: ./src/bin/analyze_common_terms.rs
// Original file: ./src/bin/analyze_common_terms.rs
// Function: hash_symbol

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

#[decl_split_decls_rs_analyze_common_terms]
fn hash_symbol (name : & str) -> u64 { let mut hasher = std :: collections :: hash_map :: DefaultHasher :: new () ; std :: hash :: Hasher :: write (& mut hasher , name . as_bytes ()) ; std :: hash :: Hasher :: finish (& hasher) & 0xFFFFFFFF }