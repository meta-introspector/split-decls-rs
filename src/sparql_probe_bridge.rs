use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::ast_reflector::{AstProbe, AstNodeType, ProbeFilter, ProbeAction, InjectionPosition};

#[derive(Debug, Serialize, Deserialize)]
pub struct SparqlToProbeConfig {
    pub queries: Vec<SparqlQuery>,
    pub probe_templates: HashMap<String, ProbeTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparqlQuery {
    pub name: String,
    pub query_type: QueryType,
    pub threshold: Option<f64>,
    pub limit: Option<usize>,
    pub target_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryType {
    MaxComplexity,
    ComplexityAbove(f64),
    FrequencyAbove(usize),
    PatternMatch(String),
    CrossLayerRelation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeTemplate {
    pub node_type: AstNodeType,
    pub action_template: String,
    pub wrapper_function: Option<String>,
    pub priority: u32,
}

#[derive(Debug)]
pub struct ComplexityResult {
    pub function_name: String,
    pub complexity: f64,
    pub file_path: String,
    pub frequency: usize,
}

pub struct SparqlProbeGenerator {
    rdf_data: HashMap<String, f64>, // function_name -> complexity
    frequency_data: HashMap<String, usize>, // function_name -> frequency
}

impl SparqlProbeGenerator {
    pub fn new() -> Self {
        Self {
            rdf_data: HashMap::new(),
            frequency_data: HashMap::new(),
        }
    }
    
    pub fn load_rdf_data(&mut self, kb_file: &str) -> Result<()> {
        let content = fs::read_to_string(kb_file)?;
        
        for line in content.lines() {
            if line.contains("hasComplexity") {
                if let Some((subject, complexity)) = self.parse_complexity_line(line) {
                    self.rdf_data.insert(subject, complexity);
                }
            }
        }
        
        println!("📊 Loaded {} complexity entries from RDF", self.rdf_data.len());
        Ok(())
    }
    
    fn parse_complexity_line(&self, line: &str) -> Option<(String, f64)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let subject = parts[0].replace("rustc:", "").replace("_", "");
            if let Ok(complexity) = parts[2].replace(";", "").parse::<f64>() {
                return Some((subject, complexity));
            }
        }
        None
    }
    
    pub fn execute_sparql_query(&self, query: &SparqlQuery) -> Vec<ComplexityResult> {
        match &query.query_type {
            QueryType::MaxComplexity => self.get_max_complexity_functions(query.limit.unwrap_or(10)),
            QueryType::ComplexityAbove(threshold) => self.get_functions_above_complexity(*threshold, query.limit.unwrap_or(50)),
            QueryType::FrequencyAbove(freq) => self.get_high_frequency_functions(*freq, query.limit.unwrap_or(20)),
            QueryType::PatternMatch(pattern) => self.get_pattern_matching_functions(pattern, query.limit.unwrap_or(30)),
            QueryType::CrossLayerRelation => self.get_cross_layer_functions(query.limit.unwrap_or(15)),
        }
    }
    
    fn get_max_complexity_functions(&self, limit: usize) -> Vec<ComplexityResult> {
        let mut results: Vec<_> = self.rdf_data.iter()
            .map(|(name, &complexity)| ComplexityResult {
                function_name: name.clone(),
                complexity,
                file_path: format!("src/{}.rs", name),
                frequency: self.frequency_data.get(name).copied().unwrap_or(1),
            })
            .collect();
        
        results.sort_by(|a, b| b.complexity.partial_cmp(&a.complexity).unwrap());
        results.truncate(limit);
        results
    }
    
    fn get_functions_above_complexity(&self, threshold: f64, limit: usize) -> Vec<ComplexityResult> {
        let mut results: Vec<_> = self.rdf_data.iter()
            .filter(|&(_, &complexity)| complexity >= threshold)
            .map(|(name, &complexity)| ComplexityResult {
                function_name: name.clone(),
                complexity,
                file_path: format!("src/{}.rs", name),
                frequency: self.frequency_data.get(name).copied().unwrap_or(1),
            })
            .collect();
        
        results.sort_by(|a, b| b.complexity.partial_cmp(&a.complexity).unwrap());
        results.truncate(limit);
        results
    }
    
    fn get_high_frequency_functions(&self, min_freq: usize, limit: usize) -> Vec<ComplexityResult> {
        let mut results: Vec<_> = self.frequency_data.iter()
            .filter(|&(_, &freq)| freq >= min_freq)
            .map(|(name, &frequency)| ComplexityResult {
                function_name: name.clone(),
                complexity: self.rdf_data.get(name).copied().unwrap_or(0.0),
                file_path: format!("src/{}.rs", name),
                frequency,
            })
            .collect();
        
        results.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        results.truncate(limit);
        results
    }
    
    fn get_pattern_matching_functions(&self, pattern: &str, limit: usize) -> Vec<ComplexityResult> {
        let results: Vec<_> = self.rdf_data.iter()
            .filter(|(name, _)| name.contains(pattern))
            .map(|(name, &complexity)| ComplexityResult {
                function_name: name.clone(),
                complexity,
                file_path: format!("src/{}.rs", name),
                frequency: self.frequency_data.get(name).copied().unwrap_or(1),
            })
            .take(limit)
            .collect();
        
        results
    }
    
    fn get_cross_layer_functions(&self, limit: usize) -> Vec<ComplexityResult> {
        let results: Vec<_> = self.rdf_data.iter()
            .filter(|(name, _)| name.contains("ast") || name.contains("hir") || name.contains("mir"))
            .map(|(name, &complexity)| ComplexityResult {
                function_name: name.clone(),
                complexity,
                file_path: format!("src/{}.rs", name),
                frequency: self.frequency_data.get(name).copied().unwrap_or(1),
            })
            .take(limit)
            .collect();
        
        results
    }
    
    pub fn generate_probes_from_query(&self, query: &SparqlQuery, template: &ProbeTemplate) -> Vec<AstProbe> {
        let results = self.execute_sparql_query(query);
        let mut probes = Vec::new();
        
        for (i, result) in results.iter().enumerate() {
            let probe_name = format!("{}_{}", query.name, i);
            
            let action = match template.wrapper_function.as_ref() {
                Some(wrapper) => ProbeAction::WrapFunction { 
                    wrapper: format!("{}({})", wrapper, result.function_name) 
                },
                None => ProbeAction::InjectCode {
                    code: format!("// Complexity: {:.2}, Frequency: {}", result.complexity, result.frequency),
                    position: InjectionPosition::Before,
                },
            };
            
            let probe = AstProbe {
                name: probe_name,
                node_type: template.node_type.clone(),
                filter: ProbeFilter {
                    name_pattern: Some(result.function_name.clone()),
                    visibility: None,
                    attributes: vec![],
                    contains_text: None,
                    complexity_threshold: Some(result.complexity),
                    layer: None,
                },
                action,
                enabled: true,
                priority: template.priority + i as u32,
            };
            
            probes.push(probe);
        }
        
        probes
    }
    
    pub fn generate_complexity_wrapper_probes(&self, top_n: usize) -> Vec<AstProbe> {
        let query = SparqlQuery {
            name: "top_complex_functions".to_string(),
            query_type: QueryType::MaxComplexity,
            threshold: None,
            limit: Some(top_n),
            target_template: "complexity_wrapper".to_string(),
        };
        
        let template = ProbeTemplate {
            node_type: AstNodeType::Function,
            action_template: "wrap_with_complexity_monitor".to_string(),
            wrapper_function: Some("complexity_monitor".to_string()),
            priority: 1,
        };
        
        self.generate_probes_from_query(&query, &template)
    }
}

pub fn create_example_sparql_config() -> SparqlToProbeConfig {
    let mut probe_templates = HashMap::new();
    
    probe_templates.insert("complexity_wrapper".to_string(), ProbeTemplate {
        node_type: AstNodeType::Function,
        action_template: "wrap_complex_function".to_string(),
        wrapper_function: Some("complexity_ping".to_string()),
        priority: 1,
    });
    
    probe_templates.insert("frequency_monitor".to_string(), ProbeTemplate {
        node_type: AstNodeType::Function,
        action_template: "monitor_frequent_function".to_string(),
        wrapper_function: Some("frequency_tracker".to_string()),
        priority: 2,
    });
    
    let queries = vec![
        SparqlQuery {
            name: "top_10_complex".to_string(),
            query_type: QueryType::MaxComplexity,
            threshold: None,
            limit: Some(10),
            target_template: "complexity_wrapper".to_string(),
        },
        SparqlQuery {
            name: "high_complexity".to_string(),
            query_type: QueryType::ComplexityAbove(8.0),
            threshold: Some(8.0),
            limit: Some(20),
            target_template: "complexity_wrapper".to_string(),
        },
        SparqlQuery {
            name: "frequent_functions".to_string(),
            query_type: QueryType::FrequencyAbove(100),
            threshold: None,
            limit: Some(15),
            target_template: "frequency_monitor".to_string(),
        },
    ];
    
    SparqlToProbeConfig {
        queries,
        probe_templates,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sparql_probe_generation() {
        let mut generator = SparqlProbeGenerator::new();
        generator.rdf_data.insert("test_function".to_string(), 12.5);
        
        let probes = generator.generate_complexity_wrapper_probes(5);
        assert!(!probes.is_empty());
        assert_eq!(probes[0].node_type, AstNodeType::Function);
    }
}
