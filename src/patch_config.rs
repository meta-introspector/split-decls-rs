
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use split_decls_types::SplitDeclsConfig;

pub fn load_config() -> Result<SplitDeclsConfig> {
    // Load from the default config file
    let config_path = "split-decls-rs.toml";
    let config_content = std::fs::read_to_string(config_path)?;
    let config: SplitDeclsConfig = toml::from_str(&config_content)?;
    Ok(config)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedWorkspaceMember {
    pub name: String,
    pub path: PathBuf, // Path relative to the generated workspace root (output/Cargo.toml)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedWorkspaceDependency {
    pub name: String,
    pub project_root_path: Option<PathBuf>, // Path relative to the main project root
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
    pub package: Option<String>, // Use if package name differs from crate name
    pub is_patch: Option<bool>, // Indicates if this dependency should go into [patch.crates-io]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedCrateDependency {
    pub crate_name: String, // The name of the crate within the generated workspace (e.g., "unimacro_derive")
    pub section: String, // e.g., "dependencies", "build-dependencies", "dev-dependencies"
    pub name: String,
    pub workspace: bool,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
    pub package: Option<String>, // Use if package name differs from crate name
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PatchConfig {
    #[serde(default)]
    pub generated_workspace_member: Vec<GeneratedWorkspaceMember>,
    #[serde(default)]
    pub generated_workspace_dependency: Vec<GeneratedWorkspaceDependency>,
    #[serde(default)]
    pub generated_crate_dependency: Vec<GeneratedCrateDependency>,
}

impl Default for PatchConfig {
    fn default() -> Self {
        Self {
            generated_workspace_member: Vec::new(),
            generated_workspace_dependency: Vec::new(),
            generated_crate_dependency: Vec::new(),
        }
    }
}

impl PatchConfig {
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        if !path.exists() {
            anyhow::bail!("Patch config file not found at {}", path.display());
        }
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
