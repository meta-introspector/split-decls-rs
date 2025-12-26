// Generated from: ./src/get_item_name.rs
// Original file: ./src/get_item_name.rs
// Function: get_item_name

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

#[decl_split_decls_rs_get_item_name]
pub fn get_item_name (item : & Item) -> Option < String > { match item { Item :: Fn (item_fn) => Some (item_fn . sig . ident . to_string ()) , Item :: Struct (item_struct) => Some (item_struct . ident . to_string ()) , Item :: Enum (item_enum) => Some (item_enum . ident . to_string ()) , Item :: Const (item_const) => Some (item_const . ident . to_string ()) , Item :: Static (item_static) => Some (item_static . ident . to_string ()) , Item :: Trait (item_trait) => Some (item_trait . ident . to_string ()) , Item :: Type (item_type) => Some (item_type . ident . to_string ()) , Item :: Union (item_union) => Some (item_union . ident . to_string ()) , Item :: Impl (item_impl) => { if let Some ((_ , path , _)) = & item_impl . trait_ { Some (format ! ("impl_for_{}" , path . to_token_stream () . to_string () . replace ("::" , "_"))) } else if let syn :: Type :: Path (type_path) = & * item_impl . self_ty { type_path . path . segments . last () . map (| s | format ! ("impl_for_{}" , s . ident . to_string ())) } else { None } } , _ => None , } }