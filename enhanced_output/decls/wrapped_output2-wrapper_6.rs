// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: execute_wrapped_patterns

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
fn execute_wrapped_patterns (rdf_state : & mut RdfStateMachine , patterns : & [String] , benchmark : bool) -> anyhow :: Result < () > { rdf_state . enter_function ("execute_patterns") ; for pattern in patterns { let result = interpret_syn_function ! (rdf_state , pattern , & format ! ("output2/wrapped-syn/src/decls/wrapped_syn_decls_{}.rs" , pattern . to_lowercase ())) ; if benchmark { println ! ("⚡ Executed: {} -> {}" , pattern , result) ; } } rdf_state . exit_function ("execute_patterns") ; Ok (()) }