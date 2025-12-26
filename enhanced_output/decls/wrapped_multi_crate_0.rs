// Generated from: ./src/wrapped_workspace_handlers/multi_crate.rs
// Original file: ./src/wrapped_workspace_handlers/multi_crate.rs
// Function: handle_multi_crate_wrapping

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

#[decl_split_decls_rs_multi_crate]
pub fn handle_multi_crate_wrapping (output_dir : & Path , patch_config : & patch_config :: PatchConfig , global_config : & SplitDeclsConfig , scan_root : & Path , dry_run : bool , verbose : bool , cargo_only : bool ,) -> Result < (String , Vec < crate :: eager_splitter :: ModuleNotFoundReport >) > { let mut final_cargo_toml_content = String :: new () ; let mut workspace_members_content = Vec :: new () ; let mut workspace_dependencies_content_str = String :: new () ; let mut patch_crates_io_content_str = String :: new () ; let mut all_collected_errors : Vec < crate :: eager_splitter :: ModuleNotFoundReport > = Vec :: new () ; let consolidated_workspace_deps_map = collect_and_format_workspace_dependencies (global_config , output_dir , scan_root) ? ; for (dep_name , dep_value) in consolidated_workspace_deps_map . iter () { workspace_dependencies_content_str . push_str (& format ! ("{} = {}
" , dep_name , format_toml_value_for_dependency_string (dep_value))) ; } use rayon :: prelude :: * ; let processed_crates : Vec < (Option < String > , Vec < crate :: eager_splitter :: ModuleNotFoundReport >) > = global_config . wrapping . crates . par_iter () . map (| crate_name | { let mut found_cargo_toml_path : Option < PathBuf > = None ; let mut crate_errors : Vec < crate :: eager_splitter :: ModuleNotFoundReport > = Vec :: new () ; if let Some (overrides) = & global_config . crate_path_overrides { if let Some (override_path) = overrides . get (crate_name) { let path_buf = PathBuf :: from (override_path) ; let candidate_path = if path_buf . is_absolute () { path_buf . join ("Cargo.toml") } else { scan_root . join (override_path) . join ("Cargo.toml") } ; if candidate_path . exists () { found_cargo_toml_path = Some (candidate_path) ; } } } let cargo_toml_path = if let Some (path) = found_cargo_toml_path { path } else { let direct_path = scan_root . join (crate_name) . join ("Cargo.toml") ; if direct_path . exists () { direct_path } else { let submodule_path = scan_root . join ("submodules") . join (crate_name) . join ("Cargo.toml") ; if submodule_path . exists () { submodule_path } else { eprintln ! ("Could not find Cargo.toml for crate '{}'" , crate_name) ; return (None , crate_errors) ; } } } ; let result = generate_wrapped_crate :: generate_wrapped_crate (output_dir , crate_name , & cargo_toml_path . parent () . unwrap () . to_path_buf () , global_config , patch_config , dry_run , cargo_only ,) ; match result { Ok (errors) => { let wrapped_crate_name = format ! ("wrapped-{}" , crate_name) ; (Some (format ! ("\"{}\"" , wrapped_crate_name)) , errors) } , Err (e) => { eprintln ! ("Error processing crate {}: {}" , crate_name , e) ; (None , crate_errors) } } }) . collect () ; for (member , errors) in processed_crates { if let Some (m) = member { workspace_members_content . push (m) ; } all_collected_errors . extend (errors) ; } eprintln ! ("DEBUG: workspace_dependencies_content_str:\n{}" , workspace_dependencies_content_str) ; for crate_name in & global_config . wrapping . crates { let mut patched_dep_table = Table :: new () ; patched_dep_table . insert ("path" . to_string () , Value :: String (format ! ("wrapped-{}" , crate_name))) ; patch_crates_io_content_str . push_str (& format ! ("{} = {}\n" , crate_name , format_toml_value_for_dependency_string (& Value :: Table (patched_dep_table)))) ; } final_cargo_toml_content = format ! (r#"[workspace]
resolver = "2"
members = [
    {}
]

[workspace.dependencies]
{}"# , workspace_members_content . join (",\n    ") , workspace_dependencies_content_str) ; if ! patch_crates_io_content_str . is_empty () { final_cargo_toml_content . push_str ("\n[patch.crates-io]\n") ; final_cargo_toml_content . push_str (& patch_crates_io_content_str) ; } Ok ((final_cargo_toml_content , all_collected_errors)) }