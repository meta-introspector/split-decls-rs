// Generated from: ./src/bin/rust_eigenmatrix.rs
// Original file: ./src/bin/rust_eigenmatrix.rs
// Function: create_parallel_eigenmatrix

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
fn create_parallel_eigenmatrix (decls : & [Declaration]) -> Result < Vec < Vec < f64 > > > { let n = decls . len () ; let progress = Arc :: new (Mutex :: new (0)) ; println ! ("🔥 FIRING UP ALL {} CORES!" , rayon :: current_num_threads ()) ; let matrix : Vec < Vec < f64 > > = (0 .. n) . into_par_iter () . map (| i | { let row : Vec < f64 > = (0 .. n) . into_par_iter () . map (| j | { if i == j { 1.0 } else { calculate_similarity (& decls [i] . content , & decls [j] . content) } }) . collect () ; { let mut p = progress . lock () . unwrap () ; * p += 1 ; if * p % 10 == 0 { println ! ("🔥 CPU MELTING: {}/{} rows computed" , * p , n) ; } } row }) . collect () ; println ! ("💥 EIGENMATRIX COMPUTATION COMPLETE!") ; Ok (matrix) }