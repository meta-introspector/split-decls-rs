// Generated from: ./src/bin/eigenmatrix.rs
// Original file: ./src/bin/eigenmatrix.rs
// Function: analyze_crate

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

#[decl_split_decls_rs_eigenmatrix]
fn analyze_crate (crate_path : & Path , name : String) -> Result < CrateMetrics > { let mut metrics = CrateMetrics { name , .. Default :: default () } ; let src_path = crate_path . join ("src") ; if ! src_path . exists () { return Ok (metrics) ; } analyze_rust_files (& src_path , & mut metrics) ? ; Ok (metrics) }