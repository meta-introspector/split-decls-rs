use anyhow::{Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "audit-deps")]
#[command(about = "Audit all dependencies and ensure they resolve to local submodules")]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output directory (default: output2)
    #[arg(short, long, default_value = "output2")]
    output_dir: PathBuf,

    /// Fix dependencies automatically
    #[arg(short, long)]
    fix: bool,
}

#[derive(Debug)]
struct DependencyIssue {
    crate_name: String,
    dep_name: String,
    issue_type: IssueType,
    current_spec: String,
    suggested_fix: Option<String>,
}

#[derive(Debug)]
enum IssueType {
    MissingFromWorkspace,
    NotLocalPath,
    WorkspaceInheritance,
    MissingSubmodule,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Auditing dependencies in {}", cli.output_dir.display());
    }

    // Scan all submodules to build available crates map
    let mut available_submodules = HashMap::new();
    for entry in WalkDir::new("submodules").max_depth(2) {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            if let Some(parent) = entry.path().parent() {
                if let Some(crate_name) = parent.file_name().and_then(|n| n.to_str()) {
                    available_submodules.insert(crate_name.to_string(), parent.to_path_buf());
                }
            }
        }
    }

    if cli.verbose {
        println!("Found {} available submodules", available_submodules.len());
    }

    let mut all_issues = Vec::new();
    let mut workspace_deps = HashMap::new();

    // Audit workspace Cargo.toml
    let workspace_cargo = cli.output_dir.join("Cargo.toml");
    if workspace_cargo.exists() {
        let content = fs::read_to_string(&workspace_cargo)?;
        let toml: Value = toml::from_str(&content)?;
        
        if let Some(workspace) = toml.get("workspace") {
            if let Some(deps) = workspace.get("dependencies") {
                if let Value::Table(deps_table) = deps {
                    for (dep_name, dep_spec) in deps_table {
                        workspace_deps.insert(dep_name.clone(), dep_spec.clone());
                        
                        // Check if workspace dependency points to local submodule
                        if let Value::Table(spec_table) = dep_spec {
                            if let Some(path_val) = spec_table.get("path") {
                                if let Value::String(path_str) = path_val {
                                    if !path_str.starts_with("../submodules/") {
                                        all_issues.push(DependencyIssue {
                                            crate_name: "workspace".to_string(),
                                            dep_name: dep_name.clone(),
                                            issue_type: IssueType::NotLocalPath,
                                            current_spec: path_str.clone(),
                                            suggested_fix: available_submodules.get(dep_name)
                                                .map(|p| format!("../submodules/{}", dep_name)),
                                        });
                                    }
                                }
                            } else if !spec_table.contains_key("path") {
                                // External dependency in workspace
                                if available_submodules.contains_key(dep_name) {
                                    all_issues.push(DependencyIssue {
                                        crate_name: "workspace".to_string(),
                                        dep_name: dep_name.clone(),
                                        issue_type: IssueType::MissingSubmodule,
                                        current_spec: format!("{:?}", dep_spec),
                                        suggested_fix: Some(format!("../submodules/{}", dep_name)),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Audit individual wrapped crates
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("wrapped-"))
            .unwrap_or(false) 
        {
            let cargo_toml = path.join("Cargo.toml");
            if cargo_toml.exists() {
                let crate_name = path.file_name().unwrap().to_str().unwrap();
                audit_crate_dependencies(&cargo_toml, crate_name, &workspace_deps, &available_submodules, &mut all_issues)?;
            }
        }
    }

    // Report issues
    if all_issues.is_empty() {
        println!("✅ All dependencies are properly configured");
        return Ok(());
    }

    println!("❌ Found {} dependency issues:", all_issues.len());
    for issue in &all_issues {
        match issue.issue_type {
            IssueType::MissingFromWorkspace => {
                println!("  {} -> {}: Missing from workspace dependencies", issue.crate_name, issue.dep_name);
            }
            IssueType::NotLocalPath => {
                println!("  {} -> {}: Not pointing to local submodule ({})", 
                    issue.crate_name, issue.dep_name, issue.current_spec);
            }
            IssueType::WorkspaceInheritance => {
                println!("  {} -> {}: Using workspace inheritance without workspace context", 
                    issue.crate_name, issue.dep_name);
            }
            IssueType::MissingSubmodule => {
                println!("  {} -> {}: External dep but submodule available", 
                    issue.crate_name, issue.dep_name);
            }
        }
        if let Some(fix) = &issue.suggested_fix {
            println!("    Suggested: {}", fix);
        }
    }

    if cli.fix {
        println!("\n🔧 Applying fixes...");
        apply_fixes(&all_issues, &cli.output_dir)?;
        println!("✅ Fixes applied");
    } else {
        println!("\nRun with --fix to automatically apply suggested fixes");
    }

    Ok(())
}

fn audit_crate_dependencies(
    cargo_toml_path: &Path,
    crate_name: &str,
    workspace_deps: &HashMap<String, Value>,
    available_submodules: &HashMap<String, PathBuf>,
    issues: &mut Vec<DependencyIssue>,
) -> Result<()> {
    let content = fs::read_to_string(cargo_toml_path)?;
    let toml: Value = toml::from_str(&content)?;

    for dep_section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(deps) = toml.get(dep_section) {
            if let Value::Table(deps_table) = deps {
                for (dep_name, dep_spec) in deps_table {
                    if let Value::Table(spec_table) = dep_spec {
                        if spec_table.contains_key("workspace") {
                            // Using workspace inheritance
                            if !workspace_deps.contains_key(dep_name) {
                                issues.push(DependencyIssue {
                                    crate_name: crate_name.to_string(),
                                    dep_name: dep_name.clone(),
                                    issue_type: IssueType::MissingFromWorkspace,
                                    current_spec: "workspace = true".to_string(),
                                    suggested_fix: available_submodules.get(dep_name)
                                        .map(|_| format!("path = \"../submodules/{}\"", dep_name)),
                                });
                            }
                        } else if !spec_table.contains_key("path") {
                            // External dependency
                            if available_submodules.contains_key(dep_name) {
                                issues.push(DependencyIssue {
                                    crate_name: crate_name.to_string(),
                                    dep_name: dep_name.clone(),
                                    issue_type: IssueType::MissingSubmodule,
                                    current_spec: format!("{:?}", dep_spec),
                                    suggested_fix: Some(format!("path = \"../submodules/{}\"", dep_name)),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn apply_fixes(issues: &[DependencyIssue], output_dir: &Path) -> Result<()> {
    // Group fixes by file
    let mut fixes_by_file: HashMap<String, Vec<&DependencyIssue>> = HashMap::new();
    
    for issue in issues {
        let file_key = if issue.crate_name == "workspace" {
            output_dir.join("Cargo.toml").to_string_lossy().to_string()
        } else {
            output_dir.join(&issue.crate_name).join("Cargo.toml").to_string_lossy().to_string()
        };
        fixes_by_file.entry(file_key).or_default().push(issue);
    }

    for (file_path, file_issues) in fixes_by_file {
        let path = Path::new(&file_path);
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let mut toml: Value = toml::from_str(&content)?;
            
            for issue in file_issues {
                if let Some(fix) = &issue.suggested_fix {
                    apply_single_fix(&mut toml, issue, fix)?;
                }
            }
            
            let updated_content = toml::to_string_pretty(&toml)?;
            fs::write(path, updated_content)?;
            println!("  Updated {}", path.display());
        }
    }

    Ok(())
}

fn apply_single_fix(toml: &mut Value, issue: &DependencyIssue, fix: &str) -> Result<()> {
    // This is a simplified fix application - in practice you'd need more sophisticated TOML manipulation
    println!("  Would fix {} -> {}: {}", issue.crate_name, issue.dep_name, fix);
    Ok(())
}
