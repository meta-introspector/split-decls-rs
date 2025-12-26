// Generated from: ./src/bin/audit_deps.rs
// Original file: ./src/bin/audit_deps.rs
// Function: apply_single_fix

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

#[decl_split_decls_rs_audit_deps]
fn apply_single_fix (toml : & mut Value , issue : & DependencyIssue , fix : & str) -> Result < () > { println ! ("  Would fix {} -> {}: {}" , issue . crate_name , issue . dep_name , fix) ; Ok (()) }