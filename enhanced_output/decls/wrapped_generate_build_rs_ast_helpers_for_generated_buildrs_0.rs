// Generated from: ./src/generate_build_rs_ast_helpers_for_generated_buildrs.rs
// Original file: ./src/generate_build_rs_ast_helpers_for_generated_buildrs.rs
// Function: generate_build_rs_ast_helpers_for_generated_buildrs

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

#[decl_split_decls_rs_generate_build_rs_ast_helpers_for_generated_buildrs]
pub fn generate_build_rs_ast_helpers_for_generated_buildrs () -> TokenStream { quote ! { # [doc = " Represents a single extracted declaration."] struct ExtractedDecl { name : String , kind : String , content : TokenStream , } # [doc = " Visitor to collect all top-level `use` statements."] # [derive (Default)] struct UseStatementCollector { uses : Vec < ItemUse >, } impl <'ast > Visit <'ast > for UseStatementCollector { fn visit_item_use (& mut self , i : &'ast ItemUse) { self . uses . push (i . clone ()) ; visit :: visit_item_use (self , i) ; } } fn get_item_name (item : & Item) -> Option < String > { match item { Item :: Fn (item_fn) => Some (item_fn . sig . ident . to_string ()) , Item :: Struct (item_struct) => Some (item_struct . ident . to_string ()) , Item :: Enum (item_enum) => Some (item_enum . ident . to_string ()) , Item :: Const (item_const) => Some (item_const . ident . to_string ()) , Item :: Static (item_static) => Some (item_static . ident . to_string ()) , Item :: Trait (item_trait) => Some (item_trait . ident . to_string ()) , Item :: Type (item_type) => Some (item_type . ident . to_string ()) , Item :: Union (item_union) => Some (item_union . ident . to_string ()) , _ => None , } } fn get_item_kind (item : & Item) -> Option <&'static str > { match item { Item :: Fn (_) => Some ("fn") , Item :: Struct (_) => Some ("struct") , Item :: Enum (_) => Some ("enum") , Item :: Const (_) => Some ("const") , Item :: Static (_) => Some ("static") , Item :: Trait (_) => Some ("trait") , Item :: Impl (_) => Some ("impl") , Item :: Type (_) => Some ("type") , Item :: Union (_) => Some ("union") , _ => None , } } } }