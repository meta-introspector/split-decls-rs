// Generated from: ./src/bin/ast_locality.rs
// Original file: ./src/bin/ast_locality.rs
// Function: find_local_dependencies

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

#[decl_split_decls_rs_ast_locality]
fn find_local_dependencies (content : & str) -> Vec < String > { let local_patterns = ["self::" , "super::" , "crate::" , "use crate" , "impl " , "struct " , "fn " , "mod " , "let " , "mut " , "match " , "if " , "for " , "while " , "loop "] ; local_patterns . par_iter () . filter (| pattern | content . contains (* pattern)) . map (| s | s . to_string ()) . collect () }