// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: analyze_enum_complexity

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
fn analyze_enum_complexity (e : & syn :: ItemEnum , file_path : & str) -> Option < ComplexityReport > { let variant_count = e . variants . len () ; let max_fields = e . variants . iter () . map (| v | match & v . fields { Fields :: Named (fields) => fields . named . len () , Fields :: Unnamed (fields) => fields . unnamed . len () , Fields :: Unit => 0 , }) . max () . unwrap_or (0) ; let nested_depth = e . variants . iter () . map (| v | calculate_nested_depth (& v . fields)) . max () . unwrap_or (0) ; let complexity = calculate_complexity_score (max_fields , variant_count , nested_depth) ; Some (ComplexityReport { name : e . ident . to_string () , complexity , field_count : max_fields , variant_count , nested_depth , file_path : file_path . to_string () , }) }