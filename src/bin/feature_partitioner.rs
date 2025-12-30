use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone, serde::Serialize)]
struct FeaturePartition {
    feature_name: String,
    dependencies: Vec<String>,
    cfg_conditions: Vec<String>,
}

#[derive(Debug)]
struct FeatureAnalyzer {
    dependency_features: HashMap<String, HashSet<String>>, // dependency -> required features
    all_discovered_features: HashSet<String>, // all features found across codebase
}

impl FeatureAnalyzer {
    fn new() -> Self {
        Self {
            dependency_features: HashMap::new(),
            all_discovered_features: HashSet::new(),
        }
    }
    
    fn analyze_dependency_features(&mut self, name_index: &HashMap<String, String>) -> Result<()> {
        println!("🔍 Analyzing feature requirements for {} dependencies", name_index.len());
        
        for (dep_name, file_path) in name_index {
            let features = self.extract_features_from_file(file_path)?;
            if !features.is_empty() {
                self.dependency_features.insert(dep_name.clone(), features.clone());
                self.all_discovered_features.extend(features);
            }
        }
        
        println!("📊 Found feature requirements for {} dependencies", self.dependency_features.len());
        println!("🏷️  Discovered {} unique features/conditions", self.all_discovered_features.len());
        
        // Print discovered features
        let mut sorted_features: Vec<_> = self.all_discovered_features.iter().collect();
        sorted_features.sort();
        println!("🔍 Discovered features:");
        for feature in sorted_features.iter().take(20) {
            println!("   - {}", feature);
        }
        if sorted_features.len() > 20 {
            println!("   ... and {} more", sorted_features.len() - 20);
        }
        
        Ok(())
    }
    
    fn extract_features_from_file(&self, file_path: &str) -> Result<HashSet<String>> {
        let mut features = HashSet::new();
        
        if let Ok(content) = fs::read_to_string(file_path) {
            // Extract all cfg(feature = "...") patterns
            let feature_regex = Regex::new(r#"#\[cfg\(feature = "([^"]+)"\)\]"#)?;
            for cap in feature_regex.captures_iter(&content) {
                if let Some(feature) = cap.get(1) {
                    features.insert(feature.as_str().to_string());
                }
            }
            
            // Extract cfg(all(feature = "...", ...)) patterns
            let complex_feature_regex = Regex::new(r#"feature = "([^"]+)""#)?;
            for cap in complex_feature_regex.captures_iter(&content) {
                if let Some(feature) = cap.get(1) {
                    features.insert(feature.as_str().to_string());
                }
            }
            
            // Extract target_arch conditions
            let arch_regex = Regex::new(r#"target_arch = "([^"]+)""#)?;
            for cap in arch_regex.captures_iter(&content) {
                if let Some(arch) = cap.get(1) {
                    features.insert(format!("target_arch_{}", arch.as_str()));
                }
            }
            
            // Extract target_os conditions
            let os_regex = Regex::new(r#"target_os = "([^"]+)""#)?;
            for cap in os_regex.captures_iter(&content) {
                if let Some(os) = cap.get(1) {
                    features.insert(format!("target_os_{}", os.as_str()));
                }
            }
            
            // Extract target_endian conditions
            let endian_regex = Regex::new(r#"target_endian = "([^"]+)""#)?;
            for cap in endian_regex.captures_iter(&content) {
                if let Some(endian) = cap.get(1) {
                    features.insert(format!("target_endian_{}", endian.as_str()));
                }
            }
            
            // Extract target_pointer_width conditions
            let pointer_regex = Regex::new(r#"target_pointer_width = "([^"]+)""#)?;
            for cap in pointer_regex.captures_iter(&content) {
                if let Some(width) = cap.get(1) {
                    features.insert(format!("target_pointer_width_{}", width.as_str()));
                }
            }
            
            // Extract simple cfg conditions like #[cfg(unix)], #[cfg(windows)]
            let simple_cfg_regex = Regex::new(r#"#\[cfg\(([a-zA-Z_][a-zA-Z0-9_]*)\)\]"#)?;
            for cap in simple_cfg_regex.captures_iter(&content) {
                if let Some(cfg_name) = cap.get(1) {
                    let cfg_str = cfg_name.as_str();
                    // Skip if it's a complex expression or already handled
                    if !cfg_str.contains("=") && !cfg_str.contains("(") && 
                       cfg_str != "test" && cfg_str != "debug_assertions" {
                        features.insert(format!("cfg_{}", cfg_str));
                    }
                }
            }
        }
        
        Ok(features)
    }
    
    fn partition_by_features(&self, resolved_terms: &[String]) -> Vec<FeaturePartition> {
        let mut partitions = HashMap::new();
        
        // Create base partition (no features)
        partitions.insert("base".to_string(), FeaturePartition {
            feature_name: "base".to_string(),
            dependencies: Vec::new(),
            cfg_conditions: Vec::new(),
        });
        
        for dep_name in resolved_terms {
            if let Some(features) = self.dependency_features.get(dep_name) {
                if features.is_empty() {
                    // No features - goes to base
                    partitions.get_mut("base").unwrap().dependencies.push(dep_name.clone());
                } else {
                    // Create feature combination key
                    let mut sorted_features: Vec<_> = features.iter().collect();
                    sorted_features.sort();
                    let feature_key = if sorted_features.len() == 1 {
                        sorted_features[0].clone()
                    } else {
                        sorted_features.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("+")
                    };
                    
                    partitions.entry(feature_key.clone()).or_insert_with(|| FeaturePartition {
                        feature_name: feature_key.clone(),
                        dependencies: Vec::new(),
                        cfg_conditions: sorted_features.iter().map(|f| {
                            if f.starts_with("target_arch_") {
                                format!("cfg(target_arch = \"{}\")", &f[12..])
                            } else if f.starts_with("target_os_") {
                                format!("cfg(target_os = \"{}\")", &f[10..])
                            } else if f.starts_with("target_endian_") {
                                format!("cfg(target_endian = \"{}\")", &f[14..])
                            } else if f.starts_with("target_pointer_width_") {
                                format!("cfg(target_pointer_width = \"{}\")", &f[21..])
                            } else if f.starts_with("cfg_") {
                                format!("cfg({})", &f[4..])
                            } else {
                                format!("cfg(feature = \"{}\")", f)
                            }
                        }).collect(),
                    }).dependencies.push(dep_name.clone());
                }
            } else {
                // No features required - goes in base partition
                partitions.get_mut("base").unwrap().dependencies.push(dep_name.clone());
            }
        }
        
        partitions.into_values().collect()
    }
    
    fn generate_feature_aware_macros(&self, partitions: &[FeaturePartition], name_index: &HashMap<String, String>) -> Result<String> {
        let mut macro_code = String::new();
        
        // Generate import macros for each partition
        for partition in partitions {
            if partition.feature_name == "base" {
                // Base dependencies - no feature gates
                for dep_name in &partition.dependencies {
                    if let Some(file_path) = name_index.get(dep_name) {
                        let safe_name = dep_name.replace("-", "_").replace(":", "_").replace("#", "_");
                        let corrected_path = if file_path.starts_with("output2/") {
                            format!("../../{}", file_path)
                        } else {
                            file_path.to_string()
                        };
                        
                        macro_code.push_str(&format!(
                            "macro_rules! import_{} {{\n    () => {{ include!(\"{}\"); }};\n}}\n\n",
                            safe_name, corrected_path
                        ));
                    }
                }
            } else {
                // Feature-gated dependencies
                let cfg_condition = if partition.cfg_conditions.len() == 1 {
                    partition.cfg_conditions[0].clone()
                } else {
                    format!("cfg(all({}))", partition.cfg_conditions.join(", "))
                };
                
                let safe_feature_name = partition.feature_name
                    .replace("+", "_")
                    .replace("-", "_")
                    .replace(":", "_")
                    .replace("#", "_");
                
                macro_code.push_str(&format!("#[{}]\nmod feature_{} {{\n", 
                    cfg_condition, safe_feature_name));
                
                for dep_name in &partition.dependencies {
                    if let Some(file_path) = name_index.get(dep_name) {
                        let safe_name = dep_name.replace("-", "_").replace(":", "_").replace("#", "_");
                        let corrected_path = if file_path.starts_with("output2/") {
                            format!("../../{}", file_path)
                        } else {
                            file_path.to_string()
                        };
                        
                        macro_code.push_str(&format!(
                            "    macro_rules! import_{} {{\n        () => {{ include!(\"{}\"); }};\n    }}\n\n",
                            safe_name, corrected_path
                        ));
                    }
                }
                
                macro_code.push_str("}\n\n");
            }
        }
        
        // Generate feature-aware mkbin macro
        macro_code.push_str("macro_rules! mkbin {\n    () => {\n");
        
        for partition in partitions {
            if partition.feature_name == "base" {
                for dep_name in &partition.dependencies {
                    let safe_name = dep_name.replace("-", "_").replace(":", "_").replace("#", "_");
                    macro_code.push_str(&format!(
                        "        mod {} {{\n            println!(\"📦 Loading: {}\");\n            import_{}!();\n        }}\n",
                        safe_name, dep_name, safe_name
                    ));
                }
            } else {
                let cfg_condition = if partition.cfg_conditions.len() == 1 {
                    partition.cfg_conditions[0].clone()
                } else {
                    format!("cfg(all({}))", partition.cfg_conditions.join(", "))
                };
                
                let safe_feature_name = partition.feature_name
                    .replace("+", "_")
                    .replace("-", "_")
                    .replace(":", "_")
                    .replace("#", "_");
                
                macro_code.push_str(&format!("        #[{}]\n        {{\n", cfg_condition));
                for dep_name in &partition.dependencies {
                    let safe_name = dep_name.replace("-", "_").replace(":", "_").replace("#", "_");
                    macro_code.push_str(&format!(
                        "            mod {} {{\n                println!(\"📦 Loading: {} ({})\");\n                feature_{}::import_{}!();\n            }}\n",
                        safe_name, dep_name, partition.feature_name, safe_feature_name, safe_name
                    ));
                }
                macro_code.push_str("        }\n");
            }
        }
        
        macro_code.push_str("    };\n}\n");
        
        Ok(macro_code)
    }
}

fn main() -> Result<()> {
    println!("🚀 Feature-aware dependency partitioning");
    
    // Load recursive dependencies
    let recursive_data = fs::read_to_string("recursive_dependencies.json")?;
    let recursive_json: serde_json::Value = serde_json::from_str(&recursive_data)?;
    
    // Load name index
    let name_index: HashMap<String, String> = serde_json::from_str(
        &fs::read_to_string("name_index.json")?
    )?;
    
    let mut analyzer = FeatureAnalyzer::new();
    analyzer.analyze_dependency_features(&name_index)?;
    
    if let Some(resolved_terms) = recursive_json.get("resolved_terms").and_then(|v| v.as_array()) {
        let resolved_terms: Vec<String> = resolved_terms
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        
        let partitions = analyzer.partition_by_features(&resolved_terms);
        
        println!("\n📊 Feature Partitions:");
        for partition in &partitions {
            println!("  🏷️  {}: {} dependencies", partition.feature_name, partition.dependencies.len());
            if !partition.cfg_conditions.is_empty() {
                println!("      Conditions: {}", partition.cfg_conditions.join(", "));
            }
        }
        
        // Generate feature-aware macros
        let macro_code = analyzer.generate_feature_aware_macros(&partitions, &name_index)?;
        fs::write("src/feature_aware_macros.rs", &macro_code)?;
        
        // Save partition analysis
        let analysis = serde_json::json!({
            "partitions": partitions,
            "total_dependencies": resolved_terms.len(),
            "discovered_features": analyzer.all_discovered_features.into_iter().collect::<Vec<_>>(),
            "feature_summary": partitions.iter().map(|p| {
                serde_json::json!({
                    "feature": p.feature_name,
                    "count": p.dependencies.len(),
                    "cfg_conditions": p.cfg_conditions
                })
            }).collect::<Vec<_>>()
        });
        
        fs::write("feature_partition_analysis.json", serde_json::to_string_pretty(&analysis)?)?;
        
        println!("✅ Generated src/feature_aware_macros.rs");
        println!("✅ Saved feature_partition_analysis.json");
    }
    
    Ok(())
}
