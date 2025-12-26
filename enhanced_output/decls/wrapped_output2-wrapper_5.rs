// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: run_rdf_analysis

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
fn run_rdf_analysis (rdf_state : & mut RdfStateMachine , crates : & [String] , output : & PathBuf) -> anyhow :: Result < () > { rdf_state . enter_function ("rdf_analysis") ; for crate_name in crates { rdf_state . capture_data ("analyzed_crate" , crate_name) ; } rdf_state . exit_function ("rdf_analysis") ; Ok (()) }