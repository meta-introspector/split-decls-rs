// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: analyze_k_levels

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

#[decl_split_decls_rs_k_theory_deps]
fn analyze_k_levels (graph : & DependencyGraph , decls : & [Declaration]) -> Result < KTheoryAnalysis > { println ! ("🧮 Computing K-theory levels 0-7...") ; let mut levels = Vec :: new () ; for k in 0 .. 8 { let level_analysis = compute_k_level (k , graph , decls) ? ; println ! ("   K{}: {} nodes, depth {}" , k , level_analysis . node_count , level_analysis . max_depth) ; levels . push (level_analysis) ; } Ok (KTheoryAnalysis { levels }) }