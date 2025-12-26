use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SplitDeclsConfig {
    /// A list of names for active overlay modules or features.
    /// This can be used to conditionally enable/disable logic in the generated build.rs.
    pub active_overlay_modules: Option<Vec<String>>,
    /// Raw Rust code to be injected as a prelude/header into each generated declaration file.
    pub custom_prelude_overlay: Option<String>,
    /// Optional path to the rustc source tree that should be processed.
    pub rustc_source_path: Option<PathBuf>,
    /// A map of crate names to a list of patch specifications.
    pub patches: Option<HashMap<String, Vec<PatchSpec>>>,
    /// A list of raw string replacements to apply to .rs content before AST parsing.
    pub string_replacements: Option<Vec<StringReplacement>>,
    /// A map of crate names to local paths for [patch.crates-io] entries.
    pub crates_io_patches: Option<HashMap<String, PathBuf>>,
    /// Optional GitHub organization to use for forking upstream repositories.
    pub github_org: Option<String>,
    /// Default branches to apply patches to for Git repositories.
    #[serde(default)]
    pub default_branches_to_patch: Vec<String>,
    /// Explicit mapping from upstream repository URLs to their fork URLs.
    #[serde(default)]
    pub repo_fork_mapping: HashMap<String, String>,
    /// A map of all workspace dependencies (from [workspace.dependencies])
    /// to their TOML Value representation.
    #[serde(default)]
    pub workspace_dependencies: HashMap<String, toml::Value>,
    /// Overrides for specific workspace dependencies.
    /// Use this to fix issues with generated dependency paths or versions without recompiling.
    #[serde(default)]
    pub workspace_dependency_overrides: HashMap<String, toml::Value>,
    /// Configuration for which crates to wrap.
    #[serde(default)]
    pub wrapping: WrappingConfig,
    /// Optional: Overrides for the path of certain crates, relative to the scan root.
    /// This is useful for crates not located directly under `scan_root/<crate_name>`
    /// or `scan_root/submodules/<crate_name>`.
    pub crate_path_overrides: Option<HashMap<String, PathBuf>>,
    /// Explicitly defined crate path mappings, used to override default path resolution logic.
    /// Key is the crate name, value is the path string relative to the current project root.
    pub explicit_crate_path_mappings: Option<HashMap<String, String>>,
}
