// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
// Function: calculate_ast_complexity

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

#[decl_split_decls_rs_ast_locality]
fn calculate_ast_complexity (content : & str) -> usize { let complexity_indicators = ["{" , "}" , "(" , ")" , "[" , "]" , "match" , "if" , "for" , "impl"] ; complexity_indicators . par_iter () . map (| indicator | content . matches (indicator) . count ()) . sum () }