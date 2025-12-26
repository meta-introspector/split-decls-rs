// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: build_dependency_graph

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
fn build_dependency_graph (decls : & [Declaration]) -> Result < DependencyGraph > { println ! ("🕸️  Building dependency graph...") ; let mut graph = HashMap :: new () ; let mut reverse_graph = HashMap :: new () ; for (i , decl) in decls . iter () . enumerate () { let deps = extract_dependencies (& decl . content) ; graph . insert (i , deps . clone ()) ; for dep in deps { reverse_graph . entry (dep) . or_insert_with (Vec :: new) . push (i) ; } } Ok (DependencyGraph { forward : graph , reverse : reverse_graph , node_count : decls . len () }) }