// Generated from: ./src/eager_splitter.rs
// Original file: ./src/eager_splitter.rs
// Function: copy_declarations_to_output

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

#[decl_split_decls_rs_eager_splitter]
# [doc = " Copies extracted declarations to the specified output directory."] pub fn copy_declarations_to_output (crate_name : & str , declarations : & HashMap < String , String > , output_base_path : & Path ,) -> Result < () > { let crate_output_dir = output_base_path . join (crate_name) . join ("src") . join ("decls") ; std :: fs :: create_dir_all (& crate_output_dir) . context (format ! ("Failed to create output directory for declarations: {}" , crate_output_dir . display ())) ? ; for (decl_name , decl_tokens_str) in declarations { let file_path = crate_output_dir . join (format ! ("{}.rs" , decl_name)) ; let initial_content = add_generated_rust_header ! (decl_tokens_str . as_str () , file ! () , line ! ()) ; match format_rust_file (& initial_content , & file_path) { Ok (formatted_content) => { std :: fs :: write (& file_path , formatted_content) . context (format ! ("Failed to write formatted declaration to {}" , file_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; std :: fs :: write (& file_path , content_with_error_comment) . context (format ! ("Failed to write unformatted declaration with error comment to {}" , file_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for '{} {}' (written to {})</blip>" , "declaration" , decl_name , file_path . display ()) ; } } } Ok (()) }