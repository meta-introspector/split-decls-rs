
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::patch_config;
use split_decls_types::SplitDeclsConfig;
use crate::paths::{CratePaths, setup_crate_paths};
use crate::generate_new_cargotoml::generate_new_cargotoml;
use crate::generate_new_lib_rs::generate_new_lib_rs;
use crate::generate_new_build_rs::generate_new_build_rs;
use crate::apply_patches_to_syntax_tree::apply_patches_to_syntax_tree;
use std::collections::HashMap;
use crate::rustfmt_utils::format_rust_file;
use crate::add_generated_rust_header;
use crate::eager_splitter;
use crate::copy_dir_recursive;

/// Generates a new workspace containing "wrapped" versions of the target crates.
/// Each wrapped crate will have its declarations eagerly split and patched.
#[allow(clippy::too_many_arguments)] // This function has many arguments, but they are all necessary.
pub fn generate_wrapped_crate(
    wrapped_workspace_root: &Path,
    original_crate_name: &str, // The original name of the crate, e.g., "unimacro_derive"
    original_crate_path: &Path, // Path to the original crate's directory (relative to project root)
    global_config: &SplitDeclsConfig,
    patch_config: &patch_config::PatchConfig, // New: Pass patch_config here
    dry_run: bool,
    cargo_only: bool,
) -> Result<Vec<eager_splitter::ModuleNotFoundReport>> { // Changed return type
    let wrapped_crate_name = format!("wrapped-{}", original_crate_name);
    println!("\n🏗️  === GENERATING WRAPPED CRATE: {} ===", wrapped_crate_name);
    println!("   📂 Workspace root: {}", wrapped_workspace_root.display());
    println!("   📦 Original crate: {}", original_crate_name);
    println!("   📁 Original path: {}", original_crate_path.display());
    println!("   🔧 Dry run: {}", dry_run);
    println!("   📋 Cargo only: {}", cargo_only);

    let wrapped_crate_path = wrapped_workspace_root.join(&wrapped_crate_name);
    println!("   📁 Wrapped crate path: {}", wrapped_crate_path.display());
    
    if !dry_run {
        println!("   📂 Creating directories...");
        fs::create_dir_all(&wrapped_crate_path)
            .context(format!("Failed to create wrapped crate directory: {}", wrapped_crate_path.display()))?;
        fs::create_dir_all(&wrapped_crate_path.join("src"))
            .context(format!("Failed to create src directory for wrapped crate: {}", wrapped_crate_path.display()))?;
        println!("   ✅ Directories created");
    } else {
        println!("   🔍 DRY-RUN: Would create directories");
    }

    // Setup CratePaths for the *wrapped* crate
    // The `old_lib_rs_path` etc. for this `CratePaths` will refer to the copies *within* the wrapped crate.
    println!("   🛠️  Setting up crate paths...");
    let wrapped_crate_paths = setup_crate_paths(&wrapped_crate_path)?;
    println!("   📄 Source files: {:?}", wrapped_crate_paths.source_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>());
    println!("   📄 Cargo.toml path: {}", wrapped_crate_paths.cargo_toml_path.display());
    println!("   📁 Decls output dir: {}", wrapped_crate_paths.decls_output_dir.display());

    // These will be the source for eager splitting within the wrapped crate context.
    let original_lib_rs_path = original_crate_path.join("src").join("lib.rs");
    let original_cargo_toml_path = original_crate_path.join("Cargo.toml");
    
    println!("   📖 Original lib.rs: {}", original_lib_rs_path.display());
    println!("   📄 Original Cargo.toml: {}", original_cargo_toml_path.display());
    println!("   📖 Lib.rs exists: {}", original_lib_rs_path.exists());
    println!("   📄 Cargo.toml exists: {}", original_cargo_toml_path.exists());

    let mut old_lib_rs_content = if original_lib_rs_path.exists() {
        println!("   📖 Reading original lib.rs...");
        let content = fs::read_to_string(&original_lib_rs_path)
            .context(format!("Failed to read original lib.rs at {}", original_lib_rs_path.display()))?;
        println!("   📏 Lib.rs size: {} bytes", content.len());
        content
    } else {
        println!("   ⚠️  No lib.rs found, using empty content");
        String::new()
    };
 
    // Generate Cargo.toml for the wrapped crate
    // This will reference the original project's crates if they are part of the original workspace
    generate_new_cargotoml(
        &original_cargo_toml_path,
        &wrapped_crate_paths.cargo_toml_path, // Output path for the new Cargo.toml
        original_crate_path,
        global_config,
        patch_config,
        dry_run,
    )?;
    
    // Generate lib.rs for the wrapped crate
    generate_new_lib_rs(&wrapped_crate_paths, dry_run)?;
    if !dry_run {
        let lib_rs_path = wrapped_crate_paths.source_files.iter()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("lib.rs"))
            .ok_or_else(|| anyhow::anyhow!("No lib.rs found in source files"))?;
        let lib_rs_content = fs::read_to_string(lib_rs_path)
            .context(format!("Failed to read generated lib.rs at {}", lib_rs_path.display()))?;
        format_rust_file(&lib_rs_content, lib_rs_path)?;
    }

    // Create a crate-specific config for writing to .split-decls-config.toml
    let mut crate_config = SplitDeclsConfig::default();
    crate_config.active_overlay_modules = global_config.active_overlay_modules.clone();
    crate_config.custom_prelude_overlay = global_config.custom_prelude_overlay.clone();
    crate_config.string_replacements = global_config.string_replacements.clone();
    crate_config.crates_io_patches = global_config.crates_io_patches.clone();

    // Filter patches relevant to this crate from the global config
    let crate_name_str = original_crate_name.to_string(); // Use original crate name for patch lookup
    if let Some(global_patches_map) = &global_config.patches {
        if let Some(patches_for_crate) = global_patches_map.get(&crate_name_str) {
            let mut new_patches_map = HashMap::new();
            new_patches_map.insert(crate_name_str.clone(), patches_for_crate.clone());
            crate_config.patches = Some(new_patches_map);
        }
    }

    let serialized_config = toml::to_string(&crate_config)
        .context("Failed to serialize SplitDeclsConfig for wrapped crate")?;
    if !dry_run {
        add_generated_header!(
            &wrapped_crate_paths.target_config_path,
            serialized_config.as_str(),
            file!(),
            line!()
        )
        .context(format!("Failed to write .split-decls-config.toml to {}", wrapped_crate_paths.target_config_path.display()))?;
    }
    println!("Wrote config to {}", wrapped_crate_paths.target_config_path.display());


    println!("   🔧 Applying string replacements...");
    // Eager Splitting Logic for the wrapped crate
    if let Some(replacements) = &global_config.string_replacements {
        println!("      📝 Found {} string replacements", replacements.len());
        for sr in replacements {
            old_lib_rs_content = old_lib_rs_content.replace(&sr.old, &sr.new);
            println!("         🔄 Applied: '{}' -> '{}'", sr.old, sr.new);
        }
    } else {
        println!("      ⏭️  No string replacements configured");
    }

    let mut module_not_found_errors: Vec<eager_splitter::ModuleNotFoundReport> = Vec::new(); // Initialize here

    // Skip expensive syn parsing if cargo_only is true
    if !cargo_only {
        println!("   🔍 Parsing lib.rs content ({} bytes)...", old_lib_rs_content.len());
        let mut syntax_tree = syn::parse_file(&old_lib_rs_content)
            .context("Failed to parse .rs content for eager splitting in wrapped crate")?;
        println!("   ✅ Parsed successfully: {} items found", syntax_tree.items.len());

        println!("   🩹 Applying patches to syntax tree...");
        apply_patches_to_syntax_tree(
            &mut syntax_tree,
            &wrapped_crate_name.replace("-", "_"), // Use wrapped crate name for patching
            &crate_config,
        )?;
        println!("   ✅ Patches applied");

        println!("   🔧 Creating hybrid CratePaths for processing...");
        // Create a hybrid CratePaths that reads from original source but writes to wrapped output
        let source_crate_paths = CratePaths {
            crate_path: original_crate_path.to_path_buf(),
            crate_name: wrapped_crate_paths.crate_name.clone(),
            source_files: wrapped_crate_paths.source_files.clone(),
            build_rs_path: wrapped_crate_paths.build_rs_path.clone(),
            cargo_toml_path: wrapped_crate_paths.cargo_toml_path.clone(),
            decls_output_dir: wrapped_crate_paths.decls_output_dir.clone(),
            target_config_path: wrapped_crate_paths.target_config_path.clone(),
            output_crate_path: wrapped_crate_paths.output_crate_path.clone(),
        };
        
        println!("   🚀 STARTING EAGER SPLITTING...");
        eager_splitter::split_and_generate_decls(
            &syntax_tree,
            &source_crate_paths,
            &crate_config,
            dry_run,
            &mut module_not_found_errors, // Pass the new parameter
        )?;
        println!("   ✅ EAGER SPLITTING COMPLETED");

        println!("   🏗️  Generating build.rs for wrapped crate...");
        // Generate build.rs for the wrapped crate (minimal version for monitoring patches)
        generate_new_build_rs(&wrapped_crate_paths, dry_run)?;
        if !dry_run {
            let build_rs_content = fs::read_to_string(&wrapped_crate_paths.build_rs_path)
                .context(format!("Failed to read generated build.rs at {}", wrapped_crate_paths.build_rs_path.display()))?;
            println!("      📝 Formatting build.rs ({} bytes)...", build_rs_content.len());
            format_rust_file(&build_rs_content, &wrapped_crate_paths.build_rs_path)?;
            println!("      ✅ build.rs formatted and written");
        } else {
            println!("      🔍 Dry-run: build.rs generation skipped");
        }
    } else {
        println!("   ⏭️  Skipping Rust parsing for {} (cargo-only mode)", wrapped_crate_name);
    }

    Ok(module_not_found_errors) // Return the collected errors
}
