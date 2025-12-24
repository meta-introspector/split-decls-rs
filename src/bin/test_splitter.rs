use anyhow::Result;
//use split_decls_rs::process_crate;
use split_decls_rs::process_crate;
use split_decls_types::SplitDeclsConfig;
use std::path::PathBuf;

macro_rules! mkwrapping {
    () => {
        split_decls_types::WrappingConfig {
            crates: vec![
                "cargo-toml-generator-types".to_string(),
                "cargo-toml-generator-macros".to_string(),
                "split-decls-types".to_string(),
                "split-decls-rs".to_string(),
                "workspace-merge".to_string(),
                "rustmacrodoc".to_string(),
                "pagerank_rs".to_string(),
                "reson".to_string(),
                "example_crate".to_string(),
                "cargo-toml-parts".to_string(),
                "lib-zos".to_string(),
                "test_crate".to_string(),
                "unimacro_derive".to_string(),
                "my_test_project".to_string(),
                "introspector_decl_common".to_string(),
                "introspector_decl_core".to_string(),
            ],
        }
    };
}

fn main() -> Result<()> {
    let test_crate_path = PathBuf::from("/mnt/data1/nix/vendor/rust/cargo2nix/unimacro_derive");
    
    // Create a minimal config
    let config = SplitDeclsConfig {
	wrapping: mkwrapping!(),
        active_overlay_modules: Some(vec![]),
        custom_prelude_overlay: Some("// Custom prelude\nuse proc_macro::TokenStream;\nuse quote::quote;\nuse syn::*;".to_string()),
        rustc_source_path: None,
        patches: Some(std::collections::HashMap::new()),
        string_replacements: None,
        crates_io_patches: Some(std::collections::HashMap::new()),
        github_org: None,
        default_branches_to_patch: vec![],
        repo_fork_mapping: std::collections::HashMap::new(),
        workspace_dependencies: std::collections::HashMap::new(),
        workspace_dependency_overrides: Default::default(),
    };
    
    println!("Running decl splitter on unimacro_derive crate: {}", test_crate_path.display());
    
    process_crate(&test_crate_path, &config, false)?;
    
    println!("Decl splitting completed successfully!");
    Ok(())
}
