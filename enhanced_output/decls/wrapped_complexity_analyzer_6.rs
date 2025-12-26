// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: analyze_rust_file

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
fn analyze_rust_file (file_path : & Path) -> Result < Vec < ComplexityReport > > { let content = fs :: read_to_string (file_path) ? ; let syntax_tree : File = syn :: parse_file (& content) ? ; let mut reports = Vec :: new () ; for item in syntax_tree . items { if let Some (report) = analyze_complexity (& item , file_path . to_string_lossy () . as_ref ()) { reports . push (report) ; } } Ok (reports) }