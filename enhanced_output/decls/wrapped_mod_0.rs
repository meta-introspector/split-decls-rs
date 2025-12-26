// Generated from: ./src/buildrs_generator/mod.rs
// Original file: ./src/buildrs_generator/mod.rs
// Function: generate_build_rs_token_stream

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

#[decl_split_decls_rs_mod]
# [doc = " Generates the TokenStream for the target build.rs file."] pub fn generate_build_rs_token_stream (decls_output_dir : & Path , crate_name_sanitized : & str ,) -> Result < TokenStream > { let decls_output_dir_lit = LitStr :: new (& decls_output_dir . display () . to_string () , Span :: call_site ()) ; let crate_name_sanitized_lit = LitStr :: new (crate_name_sanitized , Span :: call_site ()) ; let macros_ts = static_parts :: generate_build_rs_macros () ; let main_logic_ts = main_logic :: generate_main_logic_token_stream (& decls_output_dir_lit , & crate_name_sanitized_lit ,) ; let build_rs_token_stream = quote ! { use anyhow :: Context ; use anyhow :: Result ; use proc_macro2 :: { Span , TokenStream } ; use quote :: quote ; use syn :: LitStr ; use std :: fs ; use std :: path :: { Path , PathBuf } ; use std :: collections :: HashMap ; use syn :: { self , Item } ; use syn :: visit :: { self , Visit } ; use syn :: visit_mut :: { self , VisitMut } ; use split_decls_types :: { SplitDeclsConfig , PatchSpec , StringReplacement } ; use toml ; # macros_ts # main_logic_ts } ; Ok (build_rs_token_stream) }