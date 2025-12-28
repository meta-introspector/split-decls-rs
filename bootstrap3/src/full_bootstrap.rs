use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use log::warn;
use std::fs;
use toml;
use std::env;

// Enhanced mkdeclfn macro with argument value tracing
macro_rules! mkdeclfn {
    // Handle function with return type
    ($(#[$attr:meta])* pub fn $fn_name:ident($($arg_name:ident : $arg_type:ty),*) -> $ret:ty { $($body:tt)* }) => {
        $(#[$attr])*
        pub fn $fn_name($($arg_name : $arg_type),*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($fn_name));
            $(println!("   📥 {}: {:?}", stringify!($arg_name), $arg_name);)*
            let result = { $($body)* };
            println!("   ✅ Function {} completed", stringify!($fn_name));
            result
        }
    };
    // Handle function without return type
    ($(#[$attr:meta])* pub fn $fn_name:ident($($arg_name:ident : $arg_type:ty),*) { $($body:tt)* }) => {
        $(#[$attr])*
        pub fn $fn_name($($arg_name : $arg_type),*) {
            println!("🔧 Calling function: {}", stringify!($fn_name));
            $(println!("   📥 {}: {:?}", stringify!($arg_name), $arg_name);)*
            $($body)*
            println!("   ✅ Function {} completed", stringify!($fn_name));
        }
    };
    // Fallback
    ($($content:tt)*) => { 
        $($content)*
    };
}

// Define all necessary types and traits
#[derive(Debug, Clone)]
pub struct CratePaths {
    pub crate_path: PathBuf,
    pub crate_name: String,
    pub source_files: Vec<PathBuf>,
    pub build_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub decls_output_dir: PathBuf,
    pub target_config_path: PathBuf,
    pub output_crate_path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SplitDeclsConfig {
    pub patches: HashMap<String, String>,
    pub string_replacements: HashMap<String, String>,
    pub custom_prelude_overlay: String,
    pub workspace_dependencies: HashMap<String, toml::Value>,
    pub wrapping: WrappingConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WrappingConfig {
    pub crates: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PatchConfig {
    pub patches: HashMap<String, String>,
}

impl PatchConfig {
    pub fn load_from_file(_path: &Path) -> Result<Self> {
        Ok(Self::default())
    }
}

#[derive(Debug)]
pub struct ModuleNotFoundReport {
    pub module_name: String,
    pub error: String,
    pub crate_name: String,
    pub error_message: String,
    pub generated_file_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToml {
    pub dependencies: Option<HashMap<String, toml::Value>>,
    pub dev_dependencies: Option<HashMap<String, toml::Value>>,
    pub build_dependencies: Option<HashMap<String, toml::Value>>,
    pub workspace: Option<WorkspaceSection>,
    pub patch: Option<PatchSection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceSection {
    pub workspace_dependencies: Option<HashMap<String, toml::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchSection {
    pub crates_io: Option<HashMap<String, toml::Value>>,
}

fn dep_to_toml_value_iter(deps: Option<HashMap<String, toml::Value>>) -> impl Iterator<Item = (String, toml::Value)> {
    deps.unwrap_or_default().into_iter()
}

fn generate_wrapped_workspace(
    _output_dir: &Path,
    _patch_config: &PatchConfig,
    _global_config: &SplitDeclsConfig,
    _scan_root: &Path,
    _dry_run: bool,
    verbose: bool,
    _cargo_only: bool,
) -> Result<Vec<ModuleNotFoundReport>> {
    println!("🔧 Calling function: generate_wrapped_workspace");
    if verbose {
        println!("   📥 Processing workspace generation...");
    }
    println!("   ✅ Function generate_wrapped_workspace completed");
    Ok(vec![])
}

mod build_script_composer {
    use super::*;
    pub fn compose_build_script_from_parts(_parts_dir: &Path, _output_path: &Path) -> Result<()> {
        println!("🔧 Calling function: compose_build_script_from_parts");
        println!("   ✅ Function compose_build_script_from_parts completed");
        Ok(())
    }
}

// Include ALL extracted functions from output2
include!("../../output2/wrapped-split-decls-rs/src/decls/paths/fn/8/setup_crate_paths.rs");
include!("../../output2/wrapped-split-decls-rs/src/decls/main/fn/6/run_bootstrap_mode.rs");
include!("../../output2/wrapped-split-decls-rs/src/decls/main/fn/10/run_wrapped_workspace_mode.rs");

pub fn call_all_functions() -> Result<()> {
    println!("📋 Testing extracted functions with full bootstrap tracing...");
    
    let test_path = PathBuf::from("/tmp/test_crate");
    match setup_crate_paths(&test_path) {
        Ok(paths) => {
            println!("✅ setup_crate_paths executed: {}", paths.crate_name);
        }
        Err(e) => println!("⚠️  setup_crate_paths failed: {}", e),
    }
    
    // Call the REAL bootstrap function with full tracing
    println!("\n🔄 Calling REAL extracted bootstrap function...");
    match run_bootstrap_mode(true, false, None, false, false, &SplitDeclsConfig::default()) {
        Ok(_) => println!("✅ Bootstrap function completed"),
        Err(e) => println!("⚠️  Bootstrap function failed: {}", e),
    }
    
    Ok(())
}
