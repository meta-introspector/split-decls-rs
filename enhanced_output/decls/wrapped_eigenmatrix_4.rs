// Generated from: ./src/bin/eigenmatrix.rs
// Original file: ./src/bin/eigenmatrix.rs
// Function: analyze_rust_content

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

#[decl_split_decls_rs_eigenmatrix]
fn analyze_rust_content (content : & str , metrics : & mut CrateMetrics) { metrics . loc += content . lines () . count () ; for line in content . lines () { let trimmed = line . trim () ; if trimmed . starts_with ("fn ") || trimmed . contains (" fn ") { metrics . functions += 1 ; } if trimmed . starts_with ("struct ") || trimmed . contains (" struct ") { metrics . structs += 1 ; } if trimmed . starts_with ("enum ") || trimmed . contains (" enum ") { metrics . enums += 1 ; } if trimmed . starts_with ("macro_rules!") { metrics . macros += 1 ; } } }