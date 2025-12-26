// Generated from: ./src/bin/rustmacrodoc.rs
// Original file: ./src/bin/rustmacrodoc.rs
// Function: get_doc_comment

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

#[decl_split_decls_rs_rustmacrodoc]
fn get_doc_comment (attrs : & [Attribute]) -> Option < String > { let mut doc_comments = Vec :: new () ; for attr in attrs { if attr . path () . is_ident ("doc") { if let syn :: Meta :: NameValue (nv) = & attr . meta { if let syn :: Expr :: Lit (expr_lit) = & nv . value { if let Lit :: Str (lit_str) = & expr_lit . lit { doc_comments . push (lit_str . value () . trim () . to_string ()) ; } } } } } if doc_comments . is_empty () { None } else { Some (doc_comments . join ("\n")) } }