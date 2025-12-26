// Generated from: ./src/buildrs_generator/static_parts.rs
// Original file: ./src/buildrs_generator/static_parts.rs
// Function: generate_build_rs_macros

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

#[decl_split_decls_rs_static_parts]
pub fn generate_build_rs_macros () -> TokenStream { quote ! { macro_rules ! GetToken { ($ token : tt) => { syn :: token ::$ token :: new (proc_macro2 :: Span :: call_site ()) } ; } macro_rules ! mkImplCallVisitor { (calls : $ init_calls : expr) => { struct ImplCallVisitor { calls : std :: collections :: HashMap < String , std :: collections :: HashSet < String >>, } impl ImplCallVisitor { fn new () -> Self { ImplCallVisitor { calls : $ init_calls } } } impl <'ast > syn :: visit :: Visit <'ast > for ImplCallVisitor { fn visit_macro (& mut self , i : &'ast syn :: Macro) { if let Some (path_segment) = i . path . segments . last () { let path_str = path_segment . ident . to_string () ; if path_str . ends_with ("_impl") { if let Some (module_ident) = i . path . segments . first () { let module_name = module_ident . ident . to_string () ; let fn_name = path_str ; self . calls . entry (module_name) . or_default () . insert (fn_name) ; } } } syn :: visit :: visit_macro (self , i) ; } } } ; } } }