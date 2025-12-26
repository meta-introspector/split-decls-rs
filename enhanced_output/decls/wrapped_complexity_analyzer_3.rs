// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: calculate_nested_depth

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
fn calculate_nested_depth (fields : & Fields) -> u8 { match fields { Fields :: Named (fields) => { fields . named . iter () . map (| f | count_type_complexity (& f . ty)) . max () . unwrap_or (0) } , Fields :: Unnamed (fields) => { fields . unnamed . iter () . map (| f | count_type_complexity (& f . ty)) . max () . unwrap_or (0) } , Fields :: Unit => 0 , } }