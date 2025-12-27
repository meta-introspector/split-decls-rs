use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use split_decls_types::SplitDeclsConfig;

#[macro_use]
pub mod config_macros;
pub mod process_module_recursivly;
pub mod macro_analyzer_parts;
#[macro_use]
pub mod special_print;
// pub mod dwim_macros; // TODO: Fix compilation errors
pub mod output2_macro_system; // Lisp-like macro system
pub mod rdf_url_blob; // RDF URL blob state system
pub mod url_matrix; // URL matrix eigenform compression
pub mod rustc_eigenmatrix; // Rustc compiler eigenmatrix
pub mod format_generated_rust_files; // Add this
pub mod wrapped_workspace_handlers; // Add this
pub mod path_diff; // Add this
pub mod simple_crate_info; // Add this
pub mod extract_crate_info_simple; // Add this
pub mod find_all_cargo_tomls; // Add this

pub use config_macros::*;
pub mod patch_config;
pub use patch_config::load_config;

pub mod paths;
pub mod resolve_crate_path_in_submodule;
pub mod buildrs_ast_utils;
pub mod bootstrap_tracer;
pub mod bootstrap_cache;
pub mod macro_interpreter;
pub mod ast_reflector;
pub mod syn2macro;
pub mod bott_periodicity;
pub mod syn_mold;
pub mod signature_compressor;
pub mod monster_compressor;
pub mod rust_to_monster_reporter;
pub mod sparql_probe_bridge;
pub mod syscall_oracle;
pub mod syscall_decoupling_template;
pub mod ast_statistics;
pub mod syn_type_discovery;
pub mod meta_pattern_visitor;
pub mod introspect_macro;
pub mod conformal_field_theory;
pub mod cargo_guided_analysis;
pub mod buildrs_generator;
pub mod git_manager;
pub mod workspace_manager;
pub mod all_file_scanner;
pub mod eager_splitter;
pub mod extracted_decl;
pub mod generate_wrapped_workspace;
pub mod generate_wrapped_crate;
pub mod generate_new_cargotoml;
pub mod generate_new_workspace;
pub mod process_crate;
pub mod process_crates_in_path;
pub mod workflow_executor;
pub mod goal_parser;
pub mod rustfmt_utils;
pub mod source_tracker;
pub mod sparql_macros;
pub mod generate_new_lib_rs;
pub mod generate_new_build_rs;
pub mod apply_patches_to_syntax_tree;
pub mod auto_workspace_generator;
pub mod get_item_name;
pub mod get_item_kind;
pub mod crate_finder;
pub mod copy_dir_recursive;
pub mod process_dependencies_for_output_crate;

// Include split declarations
pub mod decls;
pub use decls::*;

// Re-export key functions and types
pub use extracted_decl::*;
pub use crate::paths::{CratePaths, setup_crate_paths};
pub use process_dependencies_for_output_crate::*;
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;

// Re-export split declarations
pub use crate::decls::*;
