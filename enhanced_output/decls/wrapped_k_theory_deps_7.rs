// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: calculate_node_complexity

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

#[decl_split_decls_rs_k_theory_deps]
fn calculate_node_complexity (content : & str) -> usize { let complexity_indicators = ["impl" , "trait" , "struct" , "enum" , "fn" , "match" , "if" , "for" , "while" , "generic" , "where" , "async" , "unsafe" , "macro" , "derive"] ; complexity_indicators . par_iter () . map (| indicator | content . matches (indicator) . count ()) . sum () }