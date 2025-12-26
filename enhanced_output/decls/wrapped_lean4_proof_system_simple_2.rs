// Generated from: ./src/bin/lean4_proof_system_simple.rs
// Original file: ./src/bin/lean4_proof_system_simple.rs
// Function: generate_lakefile

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

#[decl_split_decls_rs_lean4_proof_system_simple]
fn generate_lakefile () -> String { "fixme" . to_string () }