use std::path::Path;
use std::collections::HashMap;
use anyhow::Result;
use crate::url_matrix::*;
use crate::rdf_url_blob::*;

/// Rustc Eigenmatrix - Mathematical representation of the entire rustc compiler
#[derive(Debug)]
pub struct RustcEigenmatrix {
    /// Feature matrix of rustc components
    pub matrix: UrlMatrix,
    /// Rustc source analysis
    pub analysis: RustcAnalysis,
    /// Eigenform of the compiler
    pub eigenform: Option<CompilerEigenform>,
}

#[derive(Debug)]
pub struct RustcAnalysis {
    /// All rustc crates and their relationships
    pub crates: HashMap<String, CrateFeatures>,
    /// Compiler phases (parsing, analysis, codegen, etc.)
    pub phases: Vec<CompilerPhase>,
    /// Total lines of code
    pub total_loc: usize,
    /// Complexity metrics
    pub complexity: ComplexityMetrics,
}

#[derive(Debug, Clone)]
pub struct CrateFeatures {
    pub name: String,
    pub path: String,
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
    pub macros: usize,
    pub traits: usize,
    pub impls: usize,
    pub loc: usize,
    pub dependencies: Vec<String>,
}

#[derive(Debug)]
pub struct CompilerPhase {
    pub name: String,
    pub crates: Vec<String>,
    pub complexity: f64,
}

#[derive(Debug)]
pub struct ComplexityMetrics {
    pub cyclomatic: f64,
    pub cognitive: f64,
    pub halstead: f64,
    pub maintainability: f64,
}

#[derive(Debug)]
pub struct CompilerEigenform {
    /// Principal components of rustc
    pub components: Vec<PrincipalComponent>,
    /// Compressed compiler representation
    pub compressed_rustc: Vec<f64>,
    /// Reconstruction capability
    pub fidelity: f64,
}

#[derive(Debug)]
pub struct PrincipalComponent {
    pub name: String,
    pub variance_explained: f64,
    pub key_features: Vec<String>,
}

impl RustcEigenmatrix {
    /// Analyze rustc source and create eigenmatrix
    pub fn analyze_rustc(rustc_path: &Path) -> Result<Self> {
        println!("🦀 Analyzing rustc compiler source...");
        
        let analysis = Self::perform_rustc_analysis(rustc_path)?;
        let matrix = Self::create_feature_matrix(&analysis)?;
        
        println!("📊 Rustc analysis complete:");
        println!("  📦 Crates: {}", analysis.crates.len());
        println!("  🔧 Phases: {}", analysis.phases.len());
        println!("  📏 Total LOC: {}", analysis.total_loc);
        
        Ok(RustcEigenmatrix {
            matrix,
            analysis,
            eigenform: None,
        })
    }
    
    /// Perform deep analysis of rustc source
    fn perform_rustc_analysis(rustc_path: &Path) -> Result<RustcAnalysis> {
        let mut crates = HashMap::new();
        let mut total_loc = 0;
        
        // Scan rustc compiler directory
        let compiler_path = rustc_path.join("compiler");
        if compiler_path.exists() {
            for entry in std::fs::read_dir(&compiler_path)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let crate_name = entry.file_name().to_string_lossy().to_string();
                    if let Ok(features) = Self::analyze_crate(&entry.path()) {
                        total_loc += features.loc;
                        crates.insert(crate_name, features);
                    }
                }
            }
        }
        
        // Scan library directory
        let library_path = rustc_path.join("library");
        if library_path.exists() {
            for entry in std::fs::read_dir(&library_path)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let crate_name = format!("std_{}", entry.file_name().to_string_lossy());
                    if let Ok(features) = Self::analyze_crate(&entry.path()) {
                        total_loc += features.loc;
                        crates.insert(crate_name, features);
                    }
                }
            }
        }
        
        // Define compiler phases
        let phases = vec![
            CompilerPhase {
                name: "Parsing".to_string(),
                crates: vec!["rustc_parse".to_string(), "rustc_ast".to_string()],
                complexity: 0.8,
            },
            CompilerPhase {
                name: "Analysis".to_string(), 
                crates: vec!["rustc_hir".to_string(), "rustc_typeck".to_string()],
                complexity: 0.9,
            },
            CompilerPhase {
                name: "Codegen".to_string(),
                crates: vec!["rustc_codegen_llvm".to_string(), "rustc_codegen_ssa".to_string()],
                complexity: 0.7,
            },
        ];
        
        let complexity = ComplexityMetrics {
            cyclomatic: total_loc as f64 * 0.1,
            cognitive: total_loc as f64 * 0.05,
            halstead: total_loc as f64 * 0.2,
            maintainability: 100.0 - (total_loc as f64 / 10000.0),
        };
        
        Ok(RustcAnalysis {
            crates,
            phases,
            total_loc,
            complexity,
        })
    }
    
    /// Analyze individual crate features
    fn analyze_crate(crate_path: &Path) -> Result<CrateFeatures> {
        let crate_name = crate_path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let mut functions = 0;
        let mut structs = 0;
        let mut enums = 0;
        let mut macros = 0;
        let mut traits = 0;
        let mut impls = 0;
        let mut loc = 0;
        let mut dependencies = Vec::new();
        
        // Scan Rust source files
        if let Ok(entries) = std::fs::read_dir(crate_path.join("src")) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "rs") {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        loc += content.lines().count();
                        functions += content.matches("fn ").count();
                        structs += content.matches("struct ").count();
                        enums += content.matches("enum ").count();
                        macros += content.matches("macro_rules!").count();
                        traits += content.matches("trait ").count();
                        impls += content.matches("impl ").count();
                        
                        // Extract dependencies from use statements
                        for line in content.lines() {
                            if line.trim_start().starts_with("use rustc_") {
                                if let Some(dep) = line.split("::").next() {
                                    let dep = dep.trim_start_matches("use ").to_string();
                                    if !dependencies.contains(&dep) {
                                        dependencies.push(dep);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(CrateFeatures {
            name: crate_name,
            path: crate_path.to_string_lossy().to_string(),
            functions,
            structs,
            enums,
            macros,
            traits,
            impls,
            loc,
            dependencies,
        })
    }
    
    /// Create feature matrix from rustc analysis
    fn create_feature_matrix(analysis: &RustcAnalysis) -> Result<UrlMatrix> {
        let features = vec![
            "functions".to_string(),
            "structs".to_string(),
            "enums".to_string(),
            "macros".to_string(),
            "traits".to_string(),
            "impls".to_string(),
            "loc".to_string(),
            "dependencies".to_string(),
        ];
        
        let mut matrix = Vec::new();
        let mut urls = Vec::new();
        
        for (crate_name, crate_features) in &analysis.crates {
            let row = vec![
                crate_features.functions as f64,
                crate_features.structs as f64,
                crate_features.enums as f64,
                crate_features.macros as f64,
                crate_features.traits as f64,
                crate_features.impls as f64,
                crate_features.loc as f64,
                crate_features.dependencies.len() as f64,
            ];
            
            matrix.push(row);
            urls.push(format!("rustc://{}", crate_name));
        }
        
        Ok(UrlMatrix {
            matrix,
            urls,
            features,
            eigenform: None,
        })
    }
    
    /// Compute eigenform of rustc compiler
    pub fn compute_eigenform(&mut self) -> Result<()> {
        println!("🧮 Computing rustc eigenform...");
        
        // Compress matrix to eigenform
        self.matrix.compress_to_eigenform()?;
        
        if let Some(eigenform) = &self.matrix.eigenform {
            // Analyze principal components
            let mut components = Vec::new();
            
            for (i, eigenvalue) in eigenform.eigenvalues.iter().enumerate() {
                let variance_explained = eigenvalue / eigenform.eigenvalues.iter().sum::<f64>();
                
                let component_name = match i {
                    0 => "Core Language".to_string(),
                    1 => "Type System".to_string(), 
                    2 => "Code Generation".to_string(),
                    3 => "Standard Library".to_string(),
                    _ => format!("Component {}", i + 1),
                };
                
                let key_features = self.matrix.features.iter()
                    .take(3)
                    .cloned()
                    .collect();
                
                components.push(PrincipalComponent {
                    name: component_name,
                    variance_explained,
                    key_features,
                });
            }
            
            let fidelity = 1.0 - eigenform.ratio;
            
            self.eigenform = Some(CompilerEigenform {
                components,
                compressed_rustc: eigenform.compressed.clone(),
                fidelity,
            });
        }
        
        println!("✅ Rustc eigenform computed!");
        Ok(())
    }
    
    /// Generate rustc eigenmatrix URL
    pub fn to_eigenmatrix_url(&self) -> Result<String> {
        if let Some(eigenform) = &self.eigenform {
            let rustc_data = format!(
                "rustc_eigenmatrix:crates={};loc={};components={};fidelity={:.3}",
                self.analysis.crates.len(),
                self.analysis.total_loc,
                eigenform.components.len(),
                eigenform.fidelity
            );
            
            let encoded = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, rustc_data.as_bytes());
            Ok(format!("data:application/rustc-eigenmatrix+compiler;base64,{}", encoded))
        } else {
            Err(anyhow::anyhow!("Rustc eigenform not computed yet"))
        }
    }
    
    /// Print rustc eigenmatrix analysis
    pub fn print_analysis(&self) {
        println!("🦀 Rustc Eigenmatrix Analysis:");
        println!("  📦 Total crates: {}", self.analysis.crates.len());
        println!("  📏 Total LOC: {}", self.analysis.total_loc);
        println!("  🔧 Compiler phases: {}", self.analysis.phases.len());
        println!("  📊 Complexity:");
        println!("    🔄 Cyclomatic: {:.1}", self.analysis.complexity.cyclomatic);
        println!("    🧠 Cognitive: {:.1}", self.analysis.complexity.cognitive);
        println!("    📐 Halstead: {:.1}", self.analysis.complexity.halstead);
        println!("    🛠️ Maintainability: {:.1}", self.analysis.complexity.maintainability);
        
        if let Some(eigenform) = &self.eigenform {
            println!("  🧮 Eigenform:");
            println!("    📉 Compression: {:.1}%", (1.0 - eigenform.fidelity) * 100.0);
            println!("    🎯 Fidelity: {:.1}%", eigenform.fidelity * 100.0);
            println!("    🔢 Components: {}", eigenform.components.len());
            
            for (i, component) in eigenform.components.iter().take(3).enumerate() {
                println!("      {}. {}: {:.1}% variance", 
                    i + 1, component.name, component.variance_explained * 100.0);
            }
        }
    }
}
