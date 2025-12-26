// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
// Function: create_sparse_matrix

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
fn create_sparse_matrix (decls : & [Declaration] , analysis : & LocalityAnalysis) -> Result < SparseMatrix > { println ! ("🕸️  Creating sparse similarity matrix (1% global connections)...") ; let n = decls . len () ; let global_threshold = (n as f64 * 0.01) as usize ; let mut global_nodes : Vec < _ > = analysis . nodes . iter () . enumerate () . filter (| (_ , node) | node . local_ratio < 0.5) . collect () ; global_nodes . sort_by (| a , b | a . 1 . local_ratio . partial_cmp (& b . 1 . local_ratio) . unwrap ()) ; global_nodes . truncate (global_threshold) ; println ! ("🌐 Identified {} global nodes out of {}" , global_nodes . len () , n) ; let connections : Vec < (usize , usize , f64) > = global_nodes . par_iter () . flat_map (| (i , _) | { decls . par_iter () . enumerate () . filter_map (move | (j , _) | { if * i != j { let similarity = calculate_similarity (& decls [* i] . content , & decls [j] . content) ; if similarity > 0.3 { Some ((* i , j , similarity)) } else { None } } else { None } }) }) . collect () ; Ok (SparseMatrix { size : n , connections , global_nodes : global_nodes . into_iter () . map (| (i , _) | i) . collect () , }) }