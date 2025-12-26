// Generated from: ./src/bin/bootstrap-self-apply.rs
// Original file: ./src/bin/bootstrap-self-apply.rs
// Function: generate_rdf_ttl

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

#[decl_split_decls_rs_bootstrap-self-apply]
fn generate_rdf_ttl (rdf_state : & RdfStateMachine) -> String { let mut ttl = String :: from ("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n") ; ttl . push_str ("@prefix ast: <http://split-decls.rs/ast#> .\n") ; ttl . push_str ("@prefix exec: <http://split-decls.rs/execution#> .\n\n") ; for triple in & rdf_state . triples { ttl . push_str (& format ! ("ast:{} ast:{} \"{}\" .\n" , triple . subject . replace ("::" , "_") , triple . predicate . replace (":" , "_") , triple . object)) ; } ttl }