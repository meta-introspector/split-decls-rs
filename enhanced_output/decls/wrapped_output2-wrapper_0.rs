// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
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

#[decl_split_decls_rs_output2-wrapper]
fn main () -> anyhow :: Result < () > { let cli = Cli :: parse () ; let mut rdf_state = RdfStateMachine :: new () ; match cli . command { Commands :: Precompile { crate_path , all , bootstrap , output } => { println ! ("🔥 PRECOMPILE MODE") ; if bootstrap { println ! ("🚀 Applying to split-decls-rs bootstrap") ; precompile_bootstrap (& mut rdf_state , & output , cli . verbose) ? ; } else if let Some (path) = crate_path { println ! ("📦 Single crate mode: {}" , path . display ()) ; precompile_single_crate (& mut rdf_state , & path , & output , cli . verbose) ? ; } else if all { println ! ("🌐 Multi-crate mode") ; precompile_all_crates (& mut rdf_state , & output , cli . verbose) ? ; } else { println ! ("❌ Must specify --crate-path, --all, or --bootstrap") ; return Ok (()) ; } } Commands :: Inspect { crate_name , details } => { println ! ("🔍 INSPECTING: {}" , crate_name) ; inspect_wrapped_crate (& crate_name , details) ? ; } Commands :: Rdf { crates , output } => { println ! ("🔗 RDF ANALYSIS: {:?}" , crates) ; run_rdf_analysis (& mut rdf_state , & crates , & output) ? ; } Commands :: Execute { patterns , benchmark } => { println ! ("⚡ EXECUTING PATTERNS: {:?}" , patterns) ; execute_wrapped_patterns (& mut rdf_state , & patterns , benchmark) ? ; } } if ! rdf_state . triples . is_empty () { println ! ("\n🔗 RDF SUMMARY: {} triples captured" , rdf_state . triples . len ()) ; if cli . verbose { for triple in rdf_state . triples . iter () . take (5) { println ! ("  {} -> {} -> {}" , triple . subject , triple . predicate , triple . object) ; } } } Ok (()) }