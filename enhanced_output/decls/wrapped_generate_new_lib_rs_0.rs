// Generated from: ./src/generate_new_lib_rs.rs
// Original file: ./src/generate_new_lib_rs.rs
// Function: generate_new_lib_rs

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

#[decl_split_decls_rs_generate_new_lib_rs]
# [doc = " Generates the new, minimal src/lib.rs for the crate."] pub fn generate_new_lib_rs (paths : & CratePaths , dry_run : bool) -> Result < () > { let is_introspector_level = paths . crate_name . starts_with ("introspector_") ; let new_lib_rs_content = if is_introspector_level { quote ! { prelude ! () ; pub mod decls { use introspector_decl2_macros :: decl_module ; use quote :: quote ; use introspector_decl_core ; use introspector_decl_common ; use proc_macro2 ; use syn ; include ! ("decls/_decl_module_invocation.rs") ; } pub use decls ::*; } } else { quote ! { use introspector_decl2_macros :: { decl_module , prelude } ; prelude ! () ; pub mod decls { use introspector_decl2_macros :: decl_module ; use quote :: quote ; use introspector_decl_core ; use introspector_decl_common ; use proc_macro2 ; use syn ; include ! ("decls/_decl_module_invocation.rs") ; } pub use decls ::*; } } ; if dry_run { let new_path = paths . lib_rs_path . with_extension ("new") ; let initial_content = add_generated_rust_header ! (new_lib_rs_content . to_string () . as_str () , file ! () , line ! ()) ; match crate :: rustfmt_utils :: format_rust_file (& initial_content , & new_path) { Ok (formatted_content) => { fs :: write (& new_path , formatted_content) . context (format ! ("Failed to write formatted new lib.rs to {}" , new_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; fs :: write (& new_path , content_with_error_comment) . context (format ! ("Failed to write unformatted new lib.rs with error comment to {}" , new_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for new lib.rs (written to {})</blip>" , new_path . display ()) ; } } println ! ("Dry-run: Generated new src/lib.rs content to {} for crate {}" , new_path . display () , paths . crate_name) ; } else { let initial_content = add_generated_rust_header ! (new_lib_rs_content . to_string () . as_str () , file ! () , line ! ()) ; match crate :: rustfmt_utils :: format_rust_file (& initial_content , & paths . lib_rs_path) { Ok (formatted_content) => { fs :: write (& paths . lib_rs_path , formatted_content) . context (format ! ("Failed to write formatted new lib.rs to {}" , paths . lib_rs_path . display ())) ? ; } , Err (e) => { let error_comment = format ! ("// !!! Formatting failed for this module. The following code is unformatted. !!!\n\
                    // !!! Error: {} !!!\n\n" , e) ; let content_with_error_comment = error_comment + & initial_content ; fs :: write (& paths . lib_rs_path , content_with_error_comment) . context (format ! ("Failed to write unformatted new lib.rs with error comment to {}" , paths . lib_rs_path . display ())) ? ; error ! ("\n<blip style='color:red'>Formatting error for new lib.rs (written to {})</blip>" , paths . lib_rs_path . display ()) ; } } println ! ("Generated new src/lib.rs for crate {}" , paths . crate_name) ; } Ok (()) }