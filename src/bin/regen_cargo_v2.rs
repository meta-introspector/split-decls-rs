use anyhow::Result;
use lib_cargo::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use clap::Parser;
use std::collections::{HashMap, HashSet};

#[derive(clap::Parser)]
struct Cli {
    #[arg(long, default_value = "output2")]
    output_dir: PathBuf,
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    build: bool,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    analyze_only: bool,
}

#[derive(Debug)]
struct DependencyAnalysis {
    total_crates: usize,
    dependency_conflicts: Vec<ConflictReport>,
    dependency_types: HashMap<String, DependencyTypeStats>,
}

#[derive(Debug)]
struct ConflictReport {
    crate_name: String,
    dependency_name: String,
    conflict_details: Vec<String>,
}

#[derive(Debug)]
struct DependencyTypeStats {
    string_deps: usize,
    table_deps: usize,
    workspace_deps: usize,
    path_deps: usize,
    version_deps: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load split-decls-rs.toml configuration
    let content = 
    std::fs::read_to_string("split-decls-rs.toml").unwrap_or_default();
    let config: split_decls_types::SplitDeclsConfig = if content.is_empty() {
        split_decls_types::SplitDeclsConfig::default()
    } else {
        toml::from_str(&content).unwrap()
    };
    
    if cli.verbose {
        println!("Regenerating Cargo.toml files in {} using lib-cargo", cli.output_dir.display());
    }
    
    let processor = CargoProcessor::new(config.clone());
    
    // Analyze current state
    let analysis = analyze_dependencies(&cli.output_dir)?;
    print_analysis_report(&analysis);
    
    if cli.analyze_only {
        return Ok(());
    }
    
    // Collect workspace members
    let members = collect_workspace_members(&cli.output_dir)?;
    
    // Regenerate individual Cargo.toml files using lib-cargo processor
    let mut updated_crates = Vec::new();
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("wrapped-"))
            .unwrap_or(false) 
        {
            let crate_name = path.file_name().unwrap().to_str().unwrap()
                .strip_prefix("wrapped-").unwrap();
            
            if cli.verbose {
                println!("Processing Cargo.toml for {} with lib-cargo", crate_name);
            }

            let submodule_path = PathBuf::from("submodules").join(crate_name);
            if submodule_path.exists() {
                let original_cargo = submodule_path.join("Cargo.toml");
                let output_cargo = path.join("Cargo.toml");
                
                // Use lib-cargo processor instead of generate_wrapped_cargo_toml
                processor.process_cargo_toml(
                    &original_cargo,
                    &output_cargo,
                    &submodule_path,
                    cli.dry_run,
                )?;
                
                if !cli.dry_run {
                    updated_crates.push(path.to_path_buf());
                }
            }
        }
    }
    
    // Collect all dependencies from updated crates and existing config
    let mut all_deps = collect_all_workspace_dependencies(&cli.output_dir)?;
    
    // Add ALL dependencies found in analysis to workspace
    for (dep_name, stats) in &analysis.dependency_types {
        if !all_deps.contains_key(dep_name) {
            // Try to find the actual submodule path
            let submodule_path = PathBuf::from("../submodules").join(dep_name);
            let path_str = if submodule_path.join("Cargo.toml").exists() {
                format!("../submodules/{}", dep_name)
            } else {
                // Try common submodule patterns
                let patterns = [
                    format!("../submodules/{}", dep_name),
                    format!("../submodules/rust/compiler/{}", dep_name),
                    format!("../submodules/rust-analyzer/crates/{}", dep_name),
                    format!("../tools/{}", dep_name),
                    format!("../crates/{}", dep_name),
                ];
                
                patterns.into_iter()
                    .find(|p| PathBuf::from(p).join("Cargo.toml").exists())
                    .unwrap_or_else(|| format!("../submodules/{}", dep_name))
            };
            
            let mut workspace_entry = toml::map::Map::new();
            workspace_entry.insert("path".to_string(), toml::Value::String(path_str.clone()));
            all_deps.insert(dep_name.clone(), toml::Value::Table(workspace_entry));
            
            if cli.verbose {
                println!("Added missing workspace dependency: {} -> {}", dep_name, path_str);
            }
        }
    }
    
    // Add dependencies from split-decls-rs.toml configuration
    if let Some(overrides) = &config.crate_path_overrides {
        if cli.verbose {
            println!("Found {} package aliases", overrides.len());
        }
        for (name, path) in overrides {
            if cli.verbose {
                println!("Package alias: {} = {}", name, path.display());
            }
            let mut workspace_entry = toml::map::Map::new();
            workspace_entry.insert("path".to_string(), toml::Value::String(path.to_string_lossy().to_string()));
            all_deps.insert(name.clone(), toml::Value::Table(workspace_entry));
        }
    }
    
    // Generate workspace Cargo.toml using lib-cargo
    
    if cli.verbose {
        println!("✅ Cargo.toml regeneration complete");
    }
    
    // Build test if requested
    if cli.build && !cli.dry_run {
        test_build_crates(&updated_crates, cli.verbose)?;
    }
    
    Ok(())
}

fn analyze_dependencies(output_dir: &PathBuf) -> Result<DependencyAnalysis> {
    let mut analysis = DependencyAnalysis {
        total_crates: 0,
        dependency_conflicts: Vec::new(),
        dependency_types: HashMap::new(),
    };
    
    let mut global_deps: HashMap<String, Vec<(String, String)>> = HashMap::new(); // dep_name -> [(crate_name, dep_spec)]
    
    for entry in WalkDir::new(output_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("wrapped-"))
            .unwrap_or(false) 
        {
            let crate_name = path.file_name().unwrap().to_str().unwrap();
            let cargo_toml_path = path.join("Cargo.toml");
            
            if cargo_toml_path.exists() {
                analysis.total_crates += 1;
                analyze_crate_dependencies(&cargo_toml_path, crate_name, &mut analysis, &mut global_deps)?;
            }
        }
    }
    
    // Find conflicts
    for (dep_name, specs) in global_deps {
        if specs.len() > 1 {
            let unique_specs: HashSet<String> = specs.iter().map(|(_, spec)| spec.clone()).collect();
            if unique_specs.len() > 1 {
                // Find which crates have conflicting specs
                let mut conflict_details = Vec::new();
                for (crate_name, spec) in &specs {
                    conflict_details.push(format!("{}: {}", crate_name, spec));
                }
                
                analysis.dependency_conflicts.push(ConflictReport {
                    crate_name: "multiple".to_string(),
                    dependency_name: dep_name,
                    conflict_details,
                });
            }
        }
    }
    
    Ok(analysis)
}

fn analyze_crate_dependencies(
    cargo_toml_path: &PathBuf, 
    crate_name: &str, 
    analysis: &mut DependencyAnalysis,
    global_deps: &mut HashMap<String, Vec<(String, String)>>
) -> Result<()> {
    let content = 
    std::fs::read_to_string(cargo_toml_path)?;
    let cargo_toml: toml::Value = toml::from_str(&content)?;
    
    let sections = ["dependencies", "dev-dependencies", "build-dependencies"];
    
    for section in &sections {
        if let Some(deps) = cargo_toml.get(section).and_then(|v| v.as_table()) {
            for (dep_name, dep_value) in deps {
                let dep_spec = analyze_dependency_spec(dep_name, dep_value, analysis);
                global_deps.entry(dep_name.clone())
                    .or_insert_with(Vec::new)
                    .push((format!("{}[{}]", crate_name, section), dep_spec));
            }
        }
    }
    
    Ok(())
}

fn analyze_dependency_spec(dep_name: &str, dep_value: &toml::Value, analysis: &mut DependencyAnalysis) -> String {
    let stats = analysis.dependency_types.entry(dep_name.to_string()).or_insert_with(|| DependencyTypeStats {
        string_deps: 0,
        table_deps: 0,
        workspace_deps: 0,
        path_deps: 0,
        version_deps: 0,
    });
    
    match dep_value {
        toml::Value::String(version) => {
            stats.string_deps += 1;
            stats.version_deps += 1;
            format!("version=\"{}\"", version)
        }
        toml::Value::Table(table) => {
            stats.table_deps += 1;
            let mut spec_parts = Vec::new();
            
            if table.contains_key("workspace") {
                stats.workspace_deps += 1;
                spec_parts.push("workspace=true".to_string());
            }
            if let Some(path) = table.get("path") {
                stats.path_deps += 1;
                spec_parts.push(format!("path=\"{}\"", path.as_str().unwrap_or("?")));
            }
            if let Some(version) = table.get("version") {
                stats.version_deps += 1;
                spec_parts.push(format!("version=\"{}\"", version.as_str().unwrap_or("?")));
            }
            if let Some(optional) = table.get("optional") {
                spec_parts.push(format!("optional={}", optional.as_bool().unwrap_or(false)));
            }
            if let Some(features) = table.get("features") {
                spec_parts.push(format!("features={:?}", features));
            }
            
            format!("{{{}}}", spec_parts.join(", "))
        }
        _ => {
            format!("unknown: {:?}", dep_value)
        }
    }
}

fn print_analysis_report(analysis: &DependencyAnalysis) {
    println!("\n🔍 DEPENDENCY ANALYSIS REPORT");
    println!("============================");
    println!("Total crates analyzed: {}", analysis.total_crates);
    println!("Dependency conflicts found: {}", analysis.dependency_conflicts.len());
    
    if !analysis.dependency_conflicts.is_empty() {
        println!("\n❌ CONFLICTS:");
        for conflict in &analysis.dependency_conflicts {
            println!("  📦 {}", conflict.dependency_name);
            for detail in &conflict.conflict_details {
                println!("    - {}", detail);
            }
        }
    }
    
    println!("\n📊 DEPENDENCY TYPE STATISTICS:");
    let mut sorted_deps: Vec<_> = analysis.dependency_types.iter().collect();
    sorted_deps.sort_by_key(|(name, _)| name.as_str());
    
    for (dep_name, stats) in sorted_deps.iter().take(10) {
        if stats.string_deps > 0 || stats.table_deps > 0 {
            println!("  📦 {}: string={}, table={}, workspace={}, path={}, version={}", 
                dep_name, stats.string_deps, stats.table_deps, stats.workspace_deps, stats.path_deps, stats.version_deps);
        }
    }
    
    if sorted_deps.len() > 10 {
        println!("  ... and {} more dependencies", sorted_deps.len() - 10);
    }
    
    println!();
}

fn test_build_crates(crates: &[PathBuf], verbose: bool) -> Result<()> {
    if verbose {
        println!("Building updated crates...");
    }
    
    let mut error_count = 0;
    for crate_path in crates {
        let output = std::process::
    Command::new("cargo")
            .args(&["check", "--quiet"])
            .current_dir(crate_path)
            .output();
            
        match output {
            Ok(result) if !result.status.success() => {
                error_count += 1;
                let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                println!("❌ {}: {}", crate_name, String::from_utf8_lossy(&result.stderr).lines().next().unwrap_or("Build failed"));
            }
            Err(e) => {
                error_count += 1;
                let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                println!("❌ {}: {}", crate_name, e);
            }
            _ => {} // Success
        }
    }
    
    if error_count > 0 {
        println!("❌ {} crates failed to build", error_count);
        std::process::exit(101);
    } else if verbose {
        println!("✅ All crates built successfully");
    }
    
    Ok(())
}
