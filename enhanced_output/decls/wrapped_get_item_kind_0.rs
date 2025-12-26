// Generated from: ./src/get_item_kind.rs
// Original file: ./src/get_item_kind.rs
// Function: get_item_kind

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

#[decl_split_decls_rs_get_item_kind]
pub fn get_item_kind (item : & Item) -> Option < & 'static str > { match item { Item :: Fn (_) => Some ("fn") , Item :: Struct (_) => Some ("struct") , Item :: Enum (_) => Some ("enum") , Item :: Const (_) => Some ("const") , Item :: Static (_) => Some ("static") , Item :: Trait (_) => Some ("trait") , Item :: Impl (_) => Some ("impl") , Item :: Type (_) => Some ("type") , Item :: Union (_) => Some ("union") , _ => None , } }