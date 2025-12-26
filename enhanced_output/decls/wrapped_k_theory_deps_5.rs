// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: compute_node_k_level

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
fn compute_node_k_level (node : usize , graph : & DependencyGraph) -> usize { let in_degree = graph . reverse . get (& node) . map_or (0 , | v | v . len ()) ; let out_degree = graph . forward . get (& node) . map_or (0 , | v | v . len ()) ; match (in_degree , out_degree) { (0 , 0) => 0 , (0 , _) => 1 , (_ , 0) => 2 , (1 , 1) => 3 , (1 , _) | (_ , 1) => 4 , (2 , 2) => 5 , (_ , _) if in_degree + out_degree > 10 => 7 , _ => 6 , } }