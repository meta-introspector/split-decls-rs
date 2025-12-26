// Generated from: ./src/bin/rust_eigenmatrix.rs
// Original file: ./src/bin/rust_eigenmatrix.rs
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

#[decl_split_decls_rs_rust_eigenmatrix]
fn main () -> Result < () > { println ! ("🔥 PARALLEL Rust Diagonalization - 24 CPU EIGENMATRIX BEAST") ; let decls = load_all_declarations () ? ; println ! ("📊 Loaded {} declarations" , decls . len ()) ; let cache_file = "eigenmatrix_cache.json" ; if let Ok (cached) = load_cached_matrix (cache_file) { if cached . decl_names . len () == decls . len () { println ! ("⚡ Using cached matrix!") ; analyze_eigenmatrix (& cached . matrix , & decls) ; return Ok (()) ; } } println ! ("🚀 Computing {}x{} eigenmatrix with PARALLEL POWER..." , decls . len () , decls . len ()) ; let eigenmatrix = create_parallel_eigenmatrix (& decls) ? ; save_cached_matrix (cache_file , & eigenmatrix , & decls) ? ; analyze_eigenmatrix (& eigenmatrix , & decls) ; Ok (()) }