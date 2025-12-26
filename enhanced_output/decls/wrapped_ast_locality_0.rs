// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
// Function: main

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
fn main () -> Result < () > { println ! ("🧠 AST Locality Analyzer - 99% Local / 1% Global") ; let decls = load_all_declarations () ? ; println ! ("📊 Loaded {} AST nodes" , decls . len ()) ; let locality_analysis = analyze_locality (& decls) ? ; let sparse_matrix = create_sparse_matrix (& decls , & locality_analysis) ? ; report_locality_findings (& locality_analysis , & sparse_matrix) ; Ok (()) }