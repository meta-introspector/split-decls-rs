use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageField {
    pub name: String,
    pub version: String,
    pub source: String,
    pub dependencies: Vec<String>,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoLockPreservation {
    pub original_lock: PathBuf,
    pub split_decls_toml: PathBuf,
    pub bootstrap_stage: String,
    pub output2_stage: String,
    pub packages: Vec<PackageField>,
    pub preservation_map: HashMap<String, PreservationTrace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreservationTrace {
    pub original_package: PackageField,
    pub toml_reference: Option<String>,
    pub bootstrap_reference: Option<String>,
    pub output2_reference: Option<String>,
    pub preserved: bool,
    pub transformation_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoBuildOrder {
    pub crate_name: String,
    pub crate_path: PathBuf,
    pub build_order: Vec<BuildStep>,
    pub dependencies_resolved: Vec<String>,
    pub dry_run_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStep {
    pub step_number: usize,
    pub action: String, // "Compiling", "Checking", "Building"
    pub target: String,
    pub duration_estimate: Option<f64>,
}

pub struct CargoGuidedAnalysis {
    pub root_path: PathBuf,
    pub cargo_lock: CargoLockPreservation,
    pub build_orders: HashMap<String, CargoBuildOrder>,
    pub source_analysis_queue: Vec<PathBuf>,
}

impl CargoGuidedAnalysis {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        let cargo_lock_path = root_path.join("Cargo.lock");
        let split_decls_toml_path = root_path.join("submodules/split-decls-rs/split-decls-rs.toml");
        
        Ok(Self {
            root_path: root_path.clone(),
            cargo_lock: CargoLockPreservation {
                original_lock: cargo_lock_path,
                split_decls_toml: split_decls_toml_path,
                bootstrap_stage: "bootstrap".to_string(),
                output2_stage: "output2".to_string(),
                packages: Vec::new(),
                preservation_map: HashMap::new(),
            },
            build_orders: HashMap::new(),
            source_analysis_queue: Vec::new(),
        })
    }

    pub fn parse_cargo_lock(&mut self) -> Result<()> {
        println!("📦 PARSING CARGO.LOCK: {}", self.cargo_lock.original_lock.display());
        
        if !self.cargo_lock.original_lock.exists() {
            println!("  ⚠️  Cargo.lock not found, creating minimal example");
            self.create_minimal_cargo_lock()?;
        }
        
        let lock_content = #[syscall="read"]
    std::fs::read_to_string(&self.cargo_lock.original_lock)?;
        
        // Simple TOML parsing for Cargo.lock
        let lock_data: toml::Value = lock_content.parse()?;
        
        if let Some(packages) = lock_data.get("package").and_then(|p| p.as_array()) {
            for package in packages {
                if let Some(package_table) = package.as_table() {
                    let name = package_table.get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let version = package_table.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0.0.0")
                        .to_string();
                    
                    let source = package_table.get("source")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let dependencies = package_table.get("dependencies")
                        .and_then(|d| d.as_array())
                        .map(|deps| deps.iter()
                            .filter_map(|dep| dep.as_str())
                            .map(|s| s.to_string())
                            .collect())
                        .unwrap_or_default();
                    
                    let checksum = package_table.get("checksum")
                        .and_then(|c| c.as_str())
                        .map(|s| s.to_string());
                    
                    let package_field = PackageField {
                        name,
                        version,
                        source,
                        dependencies,
                        checksum,
                    };
                    
                    self.cargo_lock.packages.push(package_field);
                }
            }
        }
        
        println!("  ✅ Parsed {} packages from Cargo.lock", self.cargo_lock.packages.len());
        Ok(())
    }

    fn create_minimal_cargo_lock(&self) -> Result<()> {
        let minimal_lock = r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "split-decls-rs"
version = "0.1.0"
dependencies = [
 "anyhow",
 "serde",
 "syn",
]

[[package]]
name = "syn"
version = "2.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "abcd1234"

[[package]]
name = "proc-macro2"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "efgh5678"

[[package]]
name = "anyhow"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ijkl9012"
"#;
        #[syscall="write"]
    std::fs::write(&self.cargo_lock.original_lock, minimal_lock)?;
        Ok(())
    }

    pub fn trace_package_preservation(&mut self) -> Result<()> {
        println!("\n🔍 TRACING PACKAGE PRESERVATION");
        
        for package in &self.cargo_lock.packages.clone() {
            let mut trace = PreservationTrace {
                original_package: package.clone(),
                toml_reference: None,
                bootstrap_reference: None,
                output2_reference: None,
                preserved: false,
                transformation_steps: Vec::new(),
            };
            
            // Check split-decls-rs.toml reference
            if let Ok(toml_ref) = self.find_toml_reference(&package.name) {
                trace.toml_reference = Some(toml_ref);
                trace.transformation_steps.push(format!("Found in split-decls-rs.toml"));
            }
            
            // Check bootstrap stage
            if let Ok(bootstrap_ref) = self.find_bootstrap_reference(&package.name) {
                trace.bootstrap_reference = Some(bootstrap_ref);
                trace.transformation_steps.push(format!("Processed in bootstrap"));
            }
            
            // Check output2 stage
            if let Ok(output2_ref) = self.find_output2_reference(&package.name) {
                trace.output2_reference = Some(output2_ref);
                trace.transformation_steps.push(format!("Generated in output2"));
            }
            
            trace.preserved = trace.toml_reference.is_some() || 
                             trace.bootstrap_reference.is_some() || 
                             trace.output2_reference.is_some();
            
            println!("  📦 {}: {} steps, preserved: {}", 
                     package.name, 
                     trace.transformation_steps.len(),
                     trace.preserved);
            
            self.cargo_lock.preservation_map.insert(package.name.clone(), trace);
        }
        
        Ok(())
    }

    fn find_toml_reference(&self, package_name: &str) -> Result<String> {
        if let Ok(toml_content) = #[syscall="read"]
    std::fs::read_to_string(&self.cargo_lock.split_decls_toml) {
            if toml_content.contains(package_name) {
                return Ok(format!("Referenced in split-decls-rs.toml"));
            }
        }
        Err(anyhow::anyhow!("Not found in TOML"))
    }

    fn find_bootstrap_reference(&self, package_name: &str) -> Result<String> {
        let bootstrap_path = self.root_path.join("submodules/split-decls-rs/src");
        if let Ok(entries) = #[syscall="read"]
    std::fs::read_dir(bootstrap_path) {
            for entry in entries.flatten() {
                if let Ok(content) = #[syscall="read"]
    std::fs::read_to_string(entry.path()) {
                    if content.contains(package_name) {
                        return Ok(format!("Found in {}", entry.file_name().to_string_lossy()));
                    }
                }
            }
        }
        Err(anyhow::anyhow!("Not found in bootstrap"))
    }

    fn find_output2_reference(&self, package_name: &str) -> Result<String> {
        let output2_path = self.root_path.join("submodules/split-decls-rs/output2");
        if output2_path.exists() {
            if let Ok(entries) = #[syscall="read"]
    std::fs::read_dir(&output2_path) {
                for entry in entries.flatten() {
                    let wrapped_name = format!("wrapped-{}", package_name);
                    if entry.file_name().to_string_lossy().contains(&wrapped_name) {
                        return Ok(format!("Wrapped as {}", wrapped_name));
                    }
                }
            }
        }
        Err(anyhow::anyhow!("Not found in output2"))
    }

    pub fn capture_cargo_build_order(&mut self, crate_path: &Path) -> Result<CargoBuildOrder> {
        println!("🔨 CAPTURING BUILD ORDER: {}", crate_path.display());
        
        // Check if Cargo.toml exists
        let cargo_toml_path = crate_path.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(anyhow!("No Cargo.toml found in {}", crate_path.display()));
        }
        
        // Try cargo build --dry-run, fallback to simulated output
        let output = std::process::#[syscall="exec"]
    Command::new("cargo")
            .args(&["build", "--dry-run", "-v"])
            .current_dir(crate_path)
            .output();
        
        let dry_run_output = match output {
            Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
            Err(_) => {
                println!("  ⚠️  Cargo not available, simulating build order");
                self.simulate_build_order(crate_path)?
            }
        };
        
        let mut build_steps = Vec::new();
        let mut step_number = 0;
        
        // Parse cargo output for build steps
        for line in dry_run_output.lines() {
            if line.trim().starts_with("Compiling") || 
               line.trim().starts_with("Checking") || 
               line.trim().starts_with("Building") {
                
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    step_number += 1;
                    build_steps.push(BuildStep {
                        step_number,
                        action: parts[0].to_string(),
                        target: parts[1..].join(" "),
                        duration_estimate: Some(step_number as f64 * 0.1),
                    });
                }
            }
        }
        
        // If no steps found, create default steps
        if build_steps.is_empty() {
            build_steps.push(BuildStep {
                step_number: 1,
                action: "Compiling".to_string(),
                target: crate_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                duration_estimate: Some(1.0),
            });
        }
        
        // Extract dependencies from Cargo.toml
        let mut dependencies = Vec::new();
        
        if let Ok(toml_content) = #[syscall="read"]
    std::fs::read_to_string(&cargo_toml_path) {
            if let Ok(toml_value) = toml_content.parse::<toml::Value>() {
                if let Some(deps) = toml_value.get("dependencies").and_then(|d| d.as_table()) {
                    dependencies = deps.keys().cloned().collect();
                }
            }
        }
        
        let build_order = CargoBuildOrder {
            crate_name: crate_path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            crate_path: crate_path.to_path_buf(),
            build_order: build_steps,
            dependencies_resolved: dependencies,
            dry_run_output,
        };
        
        println!("  ✅ Captured {} build steps", build_order.build_order.len());
        Ok(build_order)
    }

    fn simulate_build_order(&self, crate_path: &Path) -> Result<String> {
        let crate_name = crate_path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        
        Ok(format!(
            "Compiling {} v0.1.0 ({})\n\
             Checking {} v0.1.0 ({})\n\
             Building {} v0.1.0 ({})",
            crate_name, crate_path.display(),
            crate_name, crate_path.display(),
            crate_name, crate_path.display()
        ))
    }

    pub fn analyze_all_crates(&mut self) -> Result<()> {
        println!("\n🔍 ANALYZING ALL CRATES IN WORKSPACE");
        
        // Analyze root crate
        let root_path = self.root_path.clone();
        if let Ok(build_order) = self.capture_cargo_build_order(&root_path) {
            self.build_orders.insert("root".to_string(), build_order);
        }
        
        // Analyze split-decls-rs
        let split_decls_path = self.root_path.join("submodules/split-decls-rs");
        if let Ok(build_order) = self.capture_cargo_build_order(&split_decls_path) {
            self.build_orders.insert("split-decls-rs".to_string(), build_order);
        }
        
        // Analyze output2 crates
        let output2_path = self.root_path.join("submodules/split-decls-rs/output2");
        if output2_path.exists() {
            if let Ok(entries) = #[syscall="read"]
    std::fs::read_dir(&output2_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let crate_name = entry.file_name().to_string_lossy().to_string();
                        if let Ok(build_order) = self.capture_cargo_build_order(&entry.path()) {
                            self.build_orders.insert(crate_name, build_order);
                        }
                    }
                }
            }
        }
        
        println!("  ✅ Analyzed {} crates total", self.build_orders.len());
        Ok(())
    }

    pub fn generate_cargo_guided_queue(&mut self) -> Result<()> {
        println!("\n📋 GENERATING CARGO-GUIDED ANALYSIS QUEUE");
        
        // Priority order: dependencies first, then dependents
        let mut queue = Vec::new();
        
        for (crate_name, build_order) in &self.build_orders {
            // Add source files in dependency order
            for step in &build_order.build_order {
                if step.action == "Compiling" {
                    let src_path = build_order.crate_path.join("src");
                    if src_path.exists() {
                        queue.push(src_path);
                    }
                }
            }
        }
        
        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        self.source_analysis_queue = queue.into_iter()
            .filter(|path| seen.insert(path.clone()))
            .collect();
        
        println!("  ✅ Generated queue with {} source directories", 
                 self.source_analysis_queue.len());
        Ok(())
    }

    pub fn create_cargo_command(&self, crate_path: &Path, command: &str) -> String {
        format!(
            "cd {} && cargo {} --dry-run -v 2>&1 | tee cargo_{}_output.txt",
            crate_path.display(),
            command,
            command.replace(" ", "_")
        )
    }

    pub fn save_preservation_report(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.cargo_lock)?;
        #[syscall="write"]
    std::fs::write(path, json)?;
        
        // Also create a summary report
        let mut summary = String::new();
        summary.push_str("# CARGO LOCK PRESERVATION REPORT\n\n");
        
        let preserved_count = self.cargo_lock.preservation_map.values()
            .filter(|trace| trace.preserved)
            .count();
        let total_count = self.cargo_lock.packages.len();
        
        summary.push_str(&format!("## Summary\n"));
        summary.push_str(&format!("- Total packages: {}\n", total_count));
        summary.push_str(&format!("- Preserved packages: {}\n", preserved_count));
        summary.push_str(&format!("- Preservation rate: {:.1}%\n\n", 
                                 (preserved_count as f64 / total_count as f64) * 100.0));
        
        summary.push_str("## Package Preservation Details\n\n");
        for (name, trace) in &self.cargo_lock.preservation_map {
            summary.push_str(&format!("### {}\n", name));
            summary.push_str(&format!("- Version: {}\n", trace.original_package.version));
            summary.push_str(&format!("- Preserved: {}\n", trace.preserved));
            summary.push_str(&format!("- Steps: {}\n", trace.transformation_steps.len()));
            for step in &trace.transformation_steps {
                summary.push_str(&format!("  - {}\n", step));
            }
            summary.push_str("\n");
        }
        
        #[syscall="write"]
    std::fs::write(path.replace(".json", "_summary.md"), summary)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_lock_parsing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut analysis = CargoGuidedAnalysis::new(temp_dir.path().to_path_buf()).unwrap();
        
        // The parse_cargo_lock function will create a minimal Cargo.lock if none exists
        assert!(analysis.parse_cargo_lock().is_ok());
        assert!(!analysis.cargo_lock.packages.is_empty());
    }
}
