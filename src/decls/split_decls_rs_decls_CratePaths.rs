use std::path::PathBuf;

/// Encapsulates all relevant file paths for a target crate.
pub struct CratePaths {
    pub crate_path: PathBuf,
    pub crate_name: String,
    pub lib_rs_path: PathBuf,
    pub old_lib_rs_path: PathBuf,
    pub build_rs_path: PathBuf,
    pub old_build_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub old_cargo_toml_path: PathBuf,
    pub decls_output_dir: PathBuf,
    pub target_config_path: PathBuf,
}