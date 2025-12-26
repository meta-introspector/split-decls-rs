// Generated from: ./src/all_file_scanner.rs
// Original file: ./src/all_file_scanner.rs
// Function: generate_wrapped_decl_with_origin

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

#[decl_split_decls_rs_all_file_scanner]
# [doc = " Generate wrapped declaration with file origin header"] pub fn generate_wrapped_decl_with_origin (item : & Item , original_file : & Path , crate_name : & str ,) -> String { let file_name = original_file . file_stem () . and_then (| s | s . to_str ()) . unwrap_or ("unknown") ; let item_name = match item { Item :: Fn (func) => func . sig . ident . to_string () , _ => "unknown" . to_string () , } ; format ! ("// Generated from: {}\n// Original file: {}\n// Function: {}\n\nuse proc_macro::TokenStream;\nuse quote::quote;\nuse syn::*;\nuse std::path::{{Path, PathBuf}};\nuse anyhow::{{Context, Result}};\nuse split_decls_types::SplitDeclsConfig;\npub use extracted_decl::*;\npub use process_crate::process_crate;\npub use process_crates_in_path::process_crates_in_path;\npub use generate_wrapped_workspace::generate_wrapped_workspace;\nprelude!{{}}\n\n#[decl_{}_{}]\n{}" , original_file . display () , original_file . display () , item_name , crate_name , file_name , item . to_token_stream ()) }