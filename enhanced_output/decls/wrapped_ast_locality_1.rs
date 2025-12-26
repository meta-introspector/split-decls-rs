// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
// Function: analyze_locality

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
fn analyze_locality (decls : & [Declaration]) -> Result < LocalityAnalysis > { println ! ("🔍 Analyzing AST locality patterns...") ; let analysis : Vec < NodeLocality > = decls . par_iter () . enumerate () . map (| (i , decl) | { let local_deps = find_local_dependencies (& decl . content) ; let global_deps = find_global_dependencies (& decl . content) ; let total_deps = local_deps . len () + global_deps . len () ; let local_ratio = if total_deps > 0 { local_deps . len () as f64 / total_deps as f64 } else { 1.0 } ; NodeLocality { index : i , name : decl . name . clone () , local_deps , global_deps , local_ratio , complexity : calculate_ast_complexity (& decl . content) , } }) . collect () ; Ok (LocalityAnalysis { nodes : analysis }) }