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

    // 1. Add dependencies from global_config.workspace_dependencies
    for (dep_name, dep_value) in global_config.workspace_dependencies.iter() {
        workspace_deps.insert(dep_name.clone(), dep_value.clone());
    }

    // 2. Apply overrides from workspace_dependency_overrides
    for (dep_name, override_value) in &global_config.workspace_dependency_overrides {
        workspace_deps.insert(dep_name.clone(), override_value.clone());
    }
    
    // 3. Add hardcoded introspector_decl2_macros if not already present
    let hardcoded_dep_name = "introspector_decl2_macros".to_string();
    if !workspace_deps.contains_key(&hardcoded_dep_name) {
        let mut hardcoded_dep_table = Table::new();
        hardcoded_dep_table.insert("path".to_string(), Value::String("../../submodules/patch-build-rs/introspector_decl2_macros".to_string()));
        workspace_deps.insert(hardcoded_dep_name, Value::Table(hardcoded_dep_table));
    }

    Ok(workspace_deps)
}
