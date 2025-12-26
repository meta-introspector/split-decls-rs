// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: analyze_struct_complexity

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
fn analyze_struct_complexity (s : & ItemStruct , file_path : & str) -> Option < ComplexityReport > { let field_count = match & s . fields { Fields :: Named (fields) => fields . named . len () , Fields :: Unnamed (fields) => fields . unnamed . len () , Fields :: Unit => 0 , } ; let nested_depth = calculate_nested_depth (& s . fields) ; let complexity = calculate_complexity_score (field_count , 0 , nested_depth) ; Some (ComplexityReport { name : s . ident . to_string () , complexity , field_count , variant_count : 0 , nested_depth , file_path : file_path . to_string () , }) }