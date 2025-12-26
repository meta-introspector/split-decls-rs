// Generated from: ./src/auto_workspace_generator.rs
// Original file: ./src/auto_workspace_generator.rs
// Function: should_skip_dir

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

#[decl_split_decls_rs_auto_workspace_generator]
fn should_skip_dir (path : & Path) -> bool { let name = path . file_name () . unwrap () . to_string_lossy () ; matches ! (name . as_ref () , "target" | ".git" | "node_modules") }