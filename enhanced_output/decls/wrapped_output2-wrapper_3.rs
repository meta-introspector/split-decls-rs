// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: precompile_all_crates

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
fn precompile_all_crates (rdf_state : & mut RdfStateMachine , output : & PathBuf , verbose : bool) -> anyhow :: Result < () > { rdf_state . enter_function ("precompile_all_crates") ; println ! ("🌐 Processing all crates in output2/") ; rdf_state . exit_function ("precompile_all_crates") ; Ok (()) }