use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Encapsulates all relevant file paths for a target crate.
pub struct CratePaths {
    pub crate_path: PathBuf,
    pub crate_name: String,
    pub source_files: Vec<PathBuf>, // List of all .rs files to process

    pub build_rs_path: PathBuf,

    pub cargo_toml_path: PathBuf, // New field for Cargo.toml

    pub decls_output_dir: PathBuf,
    pub target_config_path: PathBuf,
    pub output_crate_path: PathBuf,
}

/// Sets up and returns all relevant file paths for a given crate.
pub fn setup_crate_paths(crate_path: &Path) -> Result<CratePaths> {
    let crate_name_os_str = crate_path
        .file_name()
        .context("Crate path has no file name")?;
    let crate_name = crate_name_os_str
        .to_str()
        .context("Crate name is not valid UTF-8")?;

    let output_crate_path = crate_path.to_path_buf();

    let lib_rs_path = crate_path.join("src").join("lib.rs");
    let main_rs_path = crate_path.join("src").join("main.rs");

    println!("🔍 CHECKING PATHS for crate: {}", crate_name);
    println!("   📚 lib.rs exists: {}", lib_rs_path.exists());
    println!("   🎯 main.rs exists: {}", main_rs_path.exists());

    // Find all .rs files in src directory
    let src_dir = crate_path.join("src");
    let mut source_files = Vec::new();
    
    if src_dir.exists() {
        for entry in std::fs::read_dir(&src_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                println!("   📄 Found source file: {}", path.display());
                source_files.push(path);
            }
        }
    }
    
    println!("   📊 Total source files found: {}", source_files.len());

    let build_rs_path = crate_path.join("build.rs");

    let cargo_toml_path = crate_path.join("Cargo.toml");

    let decls_output_dir = output_crate_path.join("src").join("decls"); // Decls within the output crate
    let target_config_path = output_crate_path.join(".split-decls-config.toml"); // Config within the output crate

    Ok(CratePaths {
        crate_path: crate_path.to_path_buf(),
        crate_name: crate_name.to_string(),
        source_files, // Use the discovered source files

        build_rs_path,

        cargo_toml_path,

        decls_output_dir,
        target_config_path,
        output_crate_path,
    })
}