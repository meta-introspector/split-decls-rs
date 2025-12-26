// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: precompile_single_crate

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
fn precompile_single_crate (rdf_state : & mut RdfStateMachine , crate_path : & PathBuf , output : & PathBuf , verbose : bool) -> anyhow :: Result < () > { rdf_state . enter_function ("precompile_single_crate") ; rdf_state . capture_data ("target_crate" , & crate_path . to_string_lossy ()) ; println ! ("📦 Processing single crate: {}" , crate_path . display ()) ; rdf_state . exit_function ("precompile_single_crate") ; Ok (()) }