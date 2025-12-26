// Generated from: ./src/generate_new_build_rs.rs
// Original file: ./src/generate_new_build_rs.rs
// Function: generate_new_build_rs

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

#[decl_split_decls_rs_generate_new_build_rs]
# [doc = " Generates the new build.rs for the crate."] pub fn generate_new_build_rs (paths : & CratePaths , dry_run : bool) -> Result < () > { let build_rs_token_stream = buildrs_generator :: generate_build_rs_token_stream (& paths . decls_output_dir , & paths . crate_name . replace ("-" , "_") ,) ? ; if dry_run { let new_path = paths . build_rs_path . with_extension ("new") ; let initial_content = add_generated_rust_header ! (build_rs_token_stream . to_string () . as_str () , file ! () , line ! ()) ; match crate :: rustfmt_utils :: format_rust_file (& initial_content , & new_path) { Ok (formatted_content) => { fs :: write (& new_path , formatted_content) . context (format ! ("Failed to write formatted new build.rs to {}" , new_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; fs :: write (& new_path , content_with_error_comment) . context (format ! ("Failed to write unformatted new build.rs with error comment to {}" , new_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for new build.rs (written to {})</blip>" , new_path . display ()) ; } } println ! ("Dry-run: Generated new build.rs content to {} for crate {}" , new_path . display () , paths . crate_name) ; } else { let initial_content = add_generated_rust_header ! (build_rs_token_stream . to_string () . as_str () , file ! () , line ! ()) ; match crate :: rustfmt_utils :: format_rust_file (& initial_content , & paths . build_rs_path) { Ok (formatted_content) => { fs :: write (& paths . build_rs_path , formatted_content) . context (format ! ("Failed to write formatted new build.rs to {}" , paths . build_rs_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; fs :: write (& paths . build_rs_path , content_with_error_comment) . context (format ! ("Failed to write unformatted new build.rs with error comment to {}" , paths . build_rs_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for new build.rs (written to {})</blip>" , paths . build_rs_path . display ()) ; } } println ! ("Generated build.rs for crate {}" , paths . crate_name) ; } Ok (()) }