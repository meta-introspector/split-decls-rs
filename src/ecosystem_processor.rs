use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use split_decls_types::SplitDeclsConfig;
use walkdir::WalkDir; // Added for finding Cargo.toml files
use rayon::prelude::*; // Re-enabled for parallel processing
use crate::setup_crate_paths;
use crate::eager_splitter; // Added eager_splitter and CratePaths
use std::process::Command;
//use crate::paths::{CratePaths, setup_crate_paths};

pub fn process_ecosystem(
    verbose: bool,
    dry_run: bool,
    base_path: &Path,
    recursive: bool,
    global_config: &SplitDeclsConfig,
) -> Result<()> {
    if verbose {
        println!("Scanning ecosystem in: {}", base_path.display());
    }

    let mut cargo_toml_paths: Vec<PathBuf> = Vec::new();

    let walker = if recursive {
        WalkDir::new(base_path)
    } else {
        WalkDir::new(base_path).max_depth(1) // Only check the base_path itself
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |f| f == "Cargo.toml") {
            // Basic heuristic to avoid nested Cargo.toml files that are not crate roots
            // This could be improved with more robust logic if needed.
            if let Some(parent) = path.parent() {
                if !parent.ends_with("target") && !parent.ends_with("example_project") && !parent.ends_with("my_test_project") {
                    cargo_toml_paths.push(path.to_path_buf());
                }
            }
        }
    }

    if verbose {
        println!("Found {} Cargo.toml files.", cargo_toml_paths.len());
        println!("Splitting into 20 processes for parallel processing");
    }

    // Split cargo_toml_paths into 20 chunks for separate processes
    let chunk_size = (cargo_toml_paths.len() + 19) / 20; // Round up division
    let chunks: Vec<Vec<PathBuf>> = cargo_toml_paths
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    if verbose {
        for (i, chunk) in chunks.iter().enumerate() {
            println!("Process {}: {} crates", i + 1, chunk.len());
        }
    }

    // Spawn 20 processes to handle the chunks
    let mut handles = Vec::new();
    for (process_id, chunk) in chunks.into_iter().enumerate() {
        if chunk.is_empty() {
            continue;
        }

        let global_config = global_config.clone();
        let handle = std::thread::spawn(move || -> Result<()> {
            println!("Process {} starting with {} crates", process_id + 1, chunk.len());
            
            chunk.iter().try_for_each(|cargo_toml_path| {
                let crate_path = cargo_toml_path.parent().unwrap().to_path_buf();
                println!("Process {}: Processing crate: {}", process_id + 1, crate_path.display());

                // Setup crate paths
                let paths = setup_crate_paths(&crate_path)?;

                // Perform eager splitting
                eager_splitter::eager_split_crate(&paths, &global_config)?;

                Ok::<(), anyhow::Error>(())
            })
        });
        
        handles.push(handle);
    }

    // Wait for all processes to complete
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(result) => {
                if let Err(e) = result {
                    eprintln!("Process {} failed: {}", i + 1, e);
                }
            }
            Err(_) => {
                eprintln!("Process {} panicked", i + 1);
            }
        }
    }


    Ok(())
}
