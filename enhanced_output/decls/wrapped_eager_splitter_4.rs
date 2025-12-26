// Generated from: ./src/eager_splitter.rs
// Original file: ./src/eager_splitter.rs
// Function: eager_split_crate

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
# [doc = " Main entry point for eager splitting of a crate"] pub fn eager_split_crate (paths : & CratePaths , config : & SplitDeclsConfig) -> Result < Vec < ModuleNotFoundReport > > { println ! ("DEBUG: Entering eager_split_crate for crate: {}" , paths . crate_name) ; info ! ("📖 Parsing lib.rs...") ; let lib_content = fs :: read_to_string (& paths . lib_rs_path) . context (format ! ("Failed to read {}" , paths . lib_rs_path . display ())) ? ; info ! ("🔧 Parsing {} bytes of Rust code..." , lib_content . len ()) ; let syntax_tree : syn :: File = match syn :: parse_str (& lib_content) { Ok (ast) => ast , Err (e) => { let file_path_display = paths . lib_rs_path . display () ; let error_message = e . to_string () ; let error_line = e . span () . start () . line ; let error_column = e . span () . start () . column ; return Err (anyhow :: anyhow ! ("Failed to parse lib.rs as Rust code: {}\nFile: {}\nLine: {}, Column: {}\nError: {}" , file_path_display , paths . lib_rs_path . to_string_lossy () , error_line , error_column , error_message)) ; } } ; let mut module_not_found_errors : Vec < ModuleNotFoundReport > = Vec :: new () ; println ! ("DEBUG: Before split_and_generate_decls call.") ; split_and_generate_decls (& syntax_tree , paths , config , false , & mut module_not_found_errors) ? ; println ! ("DEBUG: After split_and_generate_decls call.") ; println ! ("DEBUG: Before generate_output_lib_rs call.") ; generate_output_lib_rs (paths) ? ; println ! ("DEBUG: After generate_output_lib_rs call.") ; println ! ("DEBUG: Before generate_new_lib_rs call.") ; generate_new_lib_rs (paths) ? ; println ! ("DEBUG: After generate_new_lib_rs call.") ; println ! ("DEBUG: Before generate_new_build_rs call.") ; generate_new_build_rs (paths) ? ; println ! ("DEBUG: After generate_new_build_rs call.") ; info ! ("Eager splitting completed for crate: {}" , paths . crate_name) ; info ! ("Output generated in: {}" , paths . decls_output_dir . parent () . unwrap () . display ()) ; println ! ("DEBUG: Exiting eager_split_crate for crate: {}" , paths . crate_name) ; Ok (module_not_found_errors) }