/// Processing audit and bisection functionality
/// 
/// This module provides the audit trail and bisection logic extracted from build.rs
/// to track and debug source code transformation issues.

use crate::parsing::test_parse;
use crate::transformations::apply_transformation_by_name;

/// Represents a single transformation step in the processing pipeline
#[derive(Debug, Clone)]
pub struct TransformationStep {
    pub name: String,
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Audit trail for processing a source file through transformations
#[derive(Debug)]
pub struct ProcessingAudit {
    pub original_content: String,
    pub steps: Vec<TransformationStep>,
    pub final_success: bool,
    pub bisection_log: Vec<String>,
}

impl ProcessingAudit {
    /// Create a new processing audit for the given content
    pub fn new(content: &str) -> Self {
        Self {
            original_content: content.to_string(),
            steps: Vec::new(),
            final_success: false,
            bisection_log: Vec::new(),
        }
    }

    /// Add a transformation step to the audit trail
    pub fn add_step(&mut self, name: &str, content: String, success: bool, error: Option<String>) {
        self.steps.push(TransformationStep {
            name: name.to_string(),
            content,
            success,
            error,
        });
    }

    /// Test if content can be parsed successfully
    pub fn test_parse(&self, content: &str) -> (bool, Option<String>) {
        test_parse(content)
    }

    /// Run bisection to find the last working transformation
    pub fn bisect_transformations(&mut self) -> Result<String, String> {
        self.bisection_log.push("Starting bisection process".to_string());
        
        // Test original
        let (original_ok, _) = self.test_parse(&self.original_content);
        if !original_ok {
            return Err("Original content doesn't parse".to_string());
        }

        // Find the last successful transformation
        let mut current = self.original_content.clone();
        for step in &self.steps {
            let test_content = apply_transformation_by_name(&current, &step.name);
            let (parse_ok, parse_error) = self.test_parse(&test_content);
            
            if parse_ok {
                current = test_content;
                self.bisection_log.push(format!("✅ {} succeeded", step.name));
            } else {
                self.bisection_log.push(format!("❌ {} failed: {:?}", step.name, parse_error));
                break;
            }
        }

        Ok(current)
    }
}
