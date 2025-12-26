// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
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

#[decl_split_decls_rs_k_theory_deps]
fn main () -> Result < () > { println ! ("🔬 8-Level K-Theory Dependency Analysis") ; let decls = load_all_declarations () ? ; println ! ("📊 Loaded {} declarations" , decls . len ()) ; let dep_graph = build_dependency_graph (& decls) ? ; let k_analysis = analyze_k_levels (& dep_graph , & decls) ? ; report_k_theory_analysis (& k_analysis) ; Ok (()) }