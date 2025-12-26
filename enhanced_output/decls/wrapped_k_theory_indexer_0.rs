// Generated from: ./src/bin/k_theory_indexer.rs
// Original file: ./src/bin/k_theory_indexer.rs
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

#[decl_split_decls_rs_k_theory_indexer]
fn main () -> Result < () > { let mut index = KIndex :: new () ; let output2_path = Path :: new ("output2") ; if ! output2_path . exists () { println ! ("No output2 directory found") ; return Ok (()) ; } let k7_files = ["complex_trait.rs" , "deep_impl.rs" , "macro_system.rs" , "type_resolver.rs" , "dependency_graph.rs" , "ast_transformer.rs" , "semantic_analyzer.rs" , "code_generator.rs"] ; for (i , file) in k7_files . iter () . enumerate () { let content = format ! ("// High complexity K7 node\npub struct {} {{}}" , file . replace (".rs" , "")) ; index . add_node (7 , (i + 1) as i32 , file . to_string () , 6.2 + i as f64 * 0.1 , 3 , format ! ("output2/{}" , file) , content) ; } let index_json = serde_json :: to_string_pretty (& index) ? ; fs :: write ("k_theory_index.json" , index_json) ? ; println ! ("🔬 K-Theory Index Generated") ; println ! ("   K7: {} nodes indexed" , k7_files . len ()) ; println ! ("   Usage: k7.1, k7.8, k7.-1") ; Ok (()) }