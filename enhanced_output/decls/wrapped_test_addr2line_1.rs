// Generated from: ./src/bin/test_addr2line.rs
// Original file: ./src/bin/test_addr2line.rs
// Function: find_addr2line_macro

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

#[decl_split_decls_rs_test_addr2line]
# [doc = " Find a specific addr2line macro by partial name match"] fn find_addr2line_macro (macros : & [(String , split_decls_rs :: output2_macro_system :: MacroDeclaration)] , target : & str) -> Option < (String , split_decls_rs :: output2_macro_system :: MacroDeclaration) > { macros . iter () . find (| (name , _) | name . contains (target)) . map (| (name , decl) | (name . clone () , decl . clone ())) }