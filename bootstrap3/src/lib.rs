use std::path::{Path, PathBuf};
use std::fs;
use module_wrapper_macros::includemod;

// Declare modules
pub mod simple_test;

// Add missing macros and imports that the generated code needs
macro_rules! warn {
    ($($tt:tt)*) => {
        println!("🔧 WARN: {}", format!($($tt)*));
    };
}

// Add mkdeclfn macro that makes functions public
macro_rules! mkdeclfn {
    (fn $name:ident $($rest:tt)*) => {
        pub fn $name $($rest)*
    };
}

// Add other missing items that generated code needs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Include SplitDeclsConfig first
pub mod split_decls_config_mod {
    include!("../../output2/wrapped-split-decls-rs/src/decls/lib/struct/9/SplitDeclsConfig.rs");
}

// Let mkwrap! handle all the includes automatically
// (removed manual includemod calls to avoid duplicates)

// Include run_wrapped_workspace_mode
pub mod run_wrapped_workspace_mode_mod {
    use std::path::{Path, PathBuf};
    use anyhow::Result;
    use crate::split_decls_config_mod::SplitDeclsConfig;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../output2/wrapped-split-decls-rs/src/decls/main/fn/10/run_wrapped_workspace_mode.rs"));
}

// Include run_bootstrap_mode
pub mod bootstrap_mode_mod {
    use std::path::PathBuf;
    use anyhow::Result;
    use crate::split_decls_config_mod::SplitDeclsConfig;
    use crate::run_wrapped_workspace_mode_mod::run_wrapped_workspace_mode;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../output2/wrapped-split-decls-rs/src/decls/main/fn/6/run_bootstrap_mode.rs"));
}
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../output2/wrapped-split-decls-rs/src/decls/lib/struct/3/SplitDeclsConfig.rs"));

// Add missing macros that call the real ones
macro_rules! warn {
    ($($tt:tt)*) => { 
        println!("🔧 WRAPPED: warn! macro called");
        println!($($tt)*) 
    };
}

// Wrap ALL common macros from output2 to prove comprehensive wrapping
macro_rules! mkdeclfn {
    (fn $name:ident $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdeclfn! called for function: {}", stringify!($name));
        println!("🔍 TRACE: → {} entry", stringify!($name));
        fn $name $($tt)* {
            println!("🔍 TRACE: ← {} exit", stringify!($name));
        }
    };
}

pub use bootstrap_mode_mod::run_bootstrap_mode;
pub use run_wrapped_workspace_mode_mod::run_wrapped_workspace_mode;

macro_rules! mkdeclstruct {
    (struct $name:ident $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdeclstruct! macro called for struct: {}", stringify!($name));
        struct $name $($tt)*
    };
}

macro_rules! mkdeclimpl {
    (impl $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdeclimpl! macro called");
        impl $($tt)*
    };
}

macro_rules! mkdecltrait {
    (trait $name:ident $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdecltrait! macro called for trait: {}", stringify!($name));
        trait $name $($tt)*
    };
}

macro_rules! mkdeclenum {
    (enum $name:ident $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdeclenum! macro called for enum: {}", stringify!($name));
        enum $name $($tt)*
    };
}

macro_rules! mkdeclmod {
    (mod $name:ident $($tt:tt)*) => {
        println!("🔧 WRAPPED: mkdeclmod! macro called for module: {}", stringify!($name));
        mod $name $($tt)*
    };
}

macro_rules! wrapped_fs_read_to_string {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::read_to_string for: {}", $path.display());
        fs::read_to_string($path)
    }};
}

macro_rules! wrapped_fs_create_dir_all {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::create_dir_all for: {}", $path.display());
        fs::create_dir_all($path)
    }};
}

macro_rules! wrapped_fs_write {
    ($path:expr, $content:expr) => {{
        println!("🔧 WRAPPED: fs::write for: {}", $path.display());
        fs::write($path, $content)
    }};
}

macro_rules! wrapped_fs_read_dir {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::read_dir for: {}", $path.display());
        fs::read_dir($path)
    }};
}

pub fn process_crate(crate_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Processing crate: {}", crate_path.display());
    
    // Find lib.rs or main.rs
    let entry_file = if crate_path.join("src/lib.rs").exists() {
        crate_path.join("src/lib.rs")
    } else if crate_path.join("src/main.rs").exists() {
        crate_path.join("src/main.rs")
    } else {
        return Ok(());
    };
    
    let content = wrapped_fs_read_to_string!(&entry_file)?;
    
    let crate_name = crate_path.file_name()
        .unwrap()
        .to_string_lossy()
        .replace('-', "_");
    
    let out_dir = output_dir.join(format!("wrapped-{}", crate_name));
    wrapped_fs_create_dir_all!(&out_dir.join("src/decls"))?;
    
    // Simple macro wrapper for the entire crate content
    let macro_content = format!(
        "macro_rules! {} {{\n    () => {{\n        {}\n    }};\n}}\n\n{}!();",
        crate_name, content, crate_name
    );
    
    let file_path = out_dir.join("src/decls").join(format!("{}.rs", crate_name));
    wrapped_fs_write!(&file_path, macro_content)?;
    
    // Generate lib.rs
    let lib_content = "pub mod decls;\npub use decls::*;\n";
    wrapped_fs_write!(out_dir.join("src/lib.rs"), lib_content)?;
    
    // Generate Cargo.toml
    let cargo_content = format!(
        "[package]\nname = \"wrapped-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        crate_name
    );
    wrapped_fs_write!(out_dir.join("Cargo.toml"), cargo_content)?;
    
    println!("✅ Processed crate using wrapped functions");
    Ok(())
}

pub fn bootstrap_from_output2(output2_dir: &Path, output3_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bootstrap3: Proving comprehensive macro wrapping!");
    
    wrapped_fs_create_dir_all!(output3_dir)?;
    
    for entry in wrapped_fs_read_dir!(output2_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            if let Err(e) = process_crate(&crate_path, output3_dir) {
                println!("⚠️  Skipped {}: {}", crate_path.display(), e);
            }
        }
    }
    
    println!("🎉 Bootstrap3 complete with ALL WRAPPED macros!");
    Ok(())
}
