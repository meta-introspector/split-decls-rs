use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

use crate::SplitDeclsConfig;
use crate::path_diff;


pub fn collect_and_format_workspace_dependencies(
    global_config: &SplitDeclsConfig,
    _output_dir: &Path,
    _scan_root: &Path,
) -> Result<HashMap<String, Value>> {
    let mut workspace_deps = HashMap::new();

    eprintln!("DEBUG: global_config.workspace_dependencies: {:?}", global_config.workspace_dependencies);
    eprintln!("DEBUG: global_config.workspace_dependency_overrides: {:?}", global_config.workspace_dependency_overrides);

    // 1. Start with dependencies from global_config.workspace_dependencies
    for (dep_name, dep_value) in global_config.workspace_dependencies.iter() {
        workspace_deps.insert(dep_name.clone(), dep_value.clone());
    }

    // 2. Add all wrapped crates as path dependencies
    for crate_name in &global_config.wrapping.crates {
        let mut dep_table = Table::new();
        dep_table.insert("path".to_string(), Value::String(format!("wrapped-{}", crate_name)));
        workspace_deps.insert(crate_name.clone(), Value::Table(dep_table));
    }

    // 3. Apply/override with explicit dependencies from workspace_dependency_overrides
    for (dep_name, override_value) in &global_config.workspace_dependency_overrides {
        workspace_deps.insert(dep_name.clone(), override_value.clone());
    }
    
    // 4. Add hardcoded introspector_decl2_macros if not already present or overridden
    let hardcoded_dep_name = "introspector_decl2_macros".to_string();
    if !workspace_deps.contains_key(&hardcoded_dep_name) {
        let mut hardcoded_dep_table = Table::new();
        hardcoded_dep_table.insert("path".to_string(), Value::String("../../submodules/patch-build-rs/introspector_decl2_macros".to_string()));
        workspace_deps.insert(hardcoded_dep_name, Value::Table(hardcoded_dep_table));
    }

    eprintln!("DEBUG: Final workspace_deps: {:?}", workspace_deps);

    Ok(workspace_deps)
}

pub fn format_toml_value_for_dependency_string(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s), // Enclose string values in quotes
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Datetime(d) => format!("\"{}\"", d), // Enclose datetime values in quotes
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(format_toml_value_for_dependency_string).collect();
            format!("[{}]", elements.join(", "))
        },
        Value::Table(table) => {
            let mut parts = Vec::new();
            for (key, val) in table.iter() {
                parts.push(format!("{} = {}", key, format_toml_value_for_dependency_string(val)));
            }
            format!("{{ {} }}", parts.join(", "))
        },
        // Fallback for other types or if to_string is acceptable
        _ => value.to_string(),
    }
}

