// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: analyze_complexity

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

#[decl_split_decls_rs_complexity_analyzer]
fn analyze_complexity (item : & Item , file_path : & str) -> Option < ComplexityReport > { match item { Item :: Struct (s) => analyze_struct_complexity (s , file_path) , Item :: Enum (e) => analyze_enum_complexity (e , file_path) , _ => None , } }