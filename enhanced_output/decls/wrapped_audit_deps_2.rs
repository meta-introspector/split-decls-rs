// Generated from: ./src/bin/audit_deps.rs
// Original file: ./src/bin/audit_deps.rs
// Function: apply_fixes

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

#[decl_split_decls_rs_audit_deps]
fn apply_fixes (issues : & [DependencyIssue] , output_dir : & Path) -> Result < () > { let mut fixes_by_file : HashMap < String , Vec < & DependencyIssue > > = HashMap :: new () ; for issue in issues { let file_key = if issue . crate_name == "workspace" { output_dir . join ("Cargo.toml") . to_string_lossy () . to_string () } else { output_dir . join (& issue . crate_name) . join ("Cargo.toml") . to_string_lossy () . to_string () } ; fixes_by_file . entry (file_key) . or_default () . push (issue) ; } for (file_path , file_issues) in fixes_by_file { let path = Path :: new (& file_path) ; if path . exists () { let content = fs :: read_to_string (path) ? ; let mut toml : Value = toml :: from_str (& content) ? ; for issue in file_issues { if let Some (fix) = & issue . suggested_fix { apply_single_fix (& mut toml , issue , fix) ? ; } } let updated_content = toml :: to_string_pretty (& toml) ? ; fs :: write (path , updated_content) ? ; println ! ("  Updated {}" , path . display ()) ; } } Ok (()) }