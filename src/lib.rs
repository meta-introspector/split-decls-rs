use std::path::{Path, PathBuf};
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
use anyhow::{Context, Result};

use split_decls_types::SplitDeclsConfig;

pub mod paths;

pub mod resolve_crate_path_in_submodule;
pub mod buildrs_ast_utils;
pub mod bootstrap_tracer;
pub mod bootstrap_cache;
pub mod macro_interpreter;
pub mod ast_reflector;
pub mod syn2macro;
pub mod syn_mold;
pub mod signature_compressor;
pub mod monster_compressor;
pub mod rust_to_monster_reporter;
pub mod monster_compressor;
pub mod sparql_probe_bridge;
pub mod syscall_oracle;
pub mod syscall_decoupling_template;
pub mod ast_statistics;
pub mod syn_type_discovery;
pub mod meta_pattern_visitor;
pub mod bott_periodicity;
pub mod conformal_field_theory;
pub mod cargo_guided_analysis;
pub mod buildrs_generator;
pub mod git_manager;
pub mod patch_config;
pub mod workspace_manager;
pub mod all_file_scanner;
pub mod eager_splitter;
pub mod extracted_decl;
pub mod generate_wrapped_workspace;
pub mod generate_wrapped_crate;
pub mod generate_new_cargotoml;
pub mod generate_new_lib_rs;
pub mod generate_new_build_rs;
pub mod apply_patches_to_syntax_tree;
pub mod auto_workspace_generator;
pub mod get_item_name;
pub mod get_item_kind;
pub mod process_crate;
pub mod generate_new_workspace;
pub mod crate_finder;
pub mod workflow_executor;
pub mod process_crates_in_path;
pub mod goal_parser;
pub mod rustfmt_utils;
pub mod copy_dir_recursive; // Add this line
pub mod source_tracker; // Add source tracking module
pub mod string_tracker; // Add string tracking module
pub use extracted_decl::*;

pub use crate::paths::{CratePaths, setup_crate_paths}; // Re-export from paths module
pub mod process_dependencies_for_output_crate;
pub use process_dependencies_for_output_crate::*;

// Re-export key functions for tests
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Package {
        name: String,
        version: String,
        edition: String,
        #[serde(default)]
        workspace: Option<bool>, // To capture package.workspace = true
        // Add other fields from your Cargo.toml package section if needed
    }
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CargoToml {
    package: Package,
    lib: Option<toml::Table>,
    #[serde(default)]
    dependencies: toml::Table, // Changed from Option<toml::Table>
    #[serde(rename = "dev-dependencies")]
    #[serde(default)]
    dev_dependencies: toml::Table, // Changed from Option<toml::Table>
    #[serde(rename = "build-dependencies")]
    #[serde(default)]
    build_dependencies: toml::Table, // Changed from Option<toml::Table>
    #[serde(flatten)]
    #[serde(default)] // Ensure `other` is initialized even if empty
    other: toml::Table,
    #[serde(default)]
    patch: toml::Table, // New field for [patch] sections
}





// Helper to convert ALL dependencies to workspace dependencies
fn process_dependency_table(
    table: &mut toml::Table,
    global_config: &SplitDeclsConfig,
    original_crate_path: &Path, // New parameter
) -> Result<()> {
    let mut deps_to_update_to_workspace = Vec::new();

    // Convert ALL dependencies to workspace = true
    for (dep_name, dep_value) in table.iter() {
        deps_to_update_to_workspace.push(dep_name.clone());
    }

    // Update all dependencies to workspace = true
    for dep_name in deps_to_update_to_workspace {
        let mut new_dep_table = toml::Table::new();
        new_dep_table.insert("workspace".to_string(), toml::Value::Boolean(true));
        
        // Preserve important fields from the original dependency
        if let Some(original_dep) = table.get(&dep_name) {
            if let Some(original_dep_table) = original_dep.as_table() {
                // Preserve features
                if let Some(features) = original_dep_table.get("features") {
                    new_dep_table.insert("features".to_string(), features.clone());
                }
                // Preserve optional flag
                if let Some(optional) = original_dep_table.get("optional") {
                    new_dep_table.insert("optional".to_string(), optional.clone());
                }
                // Preserve default-features
                if let Some(default_features) = original_dep_table.get("default-features") {
                    new_dep_table.insert("default-features".to_string(), default_features.clone());
                }
            }
        }
        table.insert(dep_name, toml::Value::Table(new_dep_table));
    }

    Ok(())
}
