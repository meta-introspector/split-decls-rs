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

    // Add all wrapped crates as path dependencies first
    for crate_name in &global_config.wrapping.crates {
        let mut dep_table = Table::new();
        dep_table.insert("path".to_string(), Value::String(format!("wrapped-{}", crate_name)));
        workspace_deps.insert(crate_name.clone(), Value::Table(dep_table));
    }

    // Then, apply entries from global_config.workspace_dependencies, but only if they don't
    // correspond to a crate that is already wrapped. This ensures wrapped versions take precedence.
    for (dep_name, dep_value) in global_config.workspace_dependencies.iter() {
        if !global_config.wrapping.crates.contains(dep_name) {
            workspace_deps.insert(dep_name.clone(), dep_value.clone());
        }
    }
    
    // Finally, apply/override with explicit dependencies from workspace_dependency_overrides
    // These take the highest precedence.
    for (dep_name, override_value) in &global_config.workspace_dependency_overrides {
        if !workspace_deps.contains_key(dep_name) {  // Prevent duplicates
            workspace_deps.insert(dep_name.clone(), override_value.clone());
        }
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

