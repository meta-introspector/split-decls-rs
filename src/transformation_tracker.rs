// Transformation tracking macros for auditable code generation

/// Macro to track transformations applied to code
/// Usage: track_transform!(content, "TRANSFORM_ID", "description", transformation_fn)
#[macro_export]
macro_rules! track_transform {
    ($content:expr, $id:expr, $desc:expr, $transform_fn:expr) => {{
        let original = $content;
        let transformed = $transform_fn(original);
        if transformed != original {
            format!("// @transform(id={}, desc=\"{}\", applied=true)\n{}", $id, $desc, transformed)
        } else {
            format!("// @transform(id={}, desc=\"{}\", applied=false)\n{}", $id, $desc, transformed)
        }
    }};
}

/// Macro to add source tracking to generated code
/// Usage: track_source!(content, file_path, line_num, generator)
#[macro_export]
macro_rules! track_source {
    ($content:expr, $file:expr, $line:expr, $gen:expr) => {{
        format!("// @source(file=\"{}\", line={}, generator=\"{}\")\n{}", $file, $line, $gen, $content)
    }};
}

/// Macro to track individual code changes within transformations
/// Usage: track_change!(old_code, new_code, "CHANGE_ID", "reason")
#[macro_export]
macro_rules! track_change {
    ($old:expr, $new:expr, $id:expr, $reason:expr) => {{
        if $old != $new {
            format!("// @change(id={}, reason=\"{}\", changed=true)\n{}", $id, $reason, $new)
        } else {
            format!("// @change(id={}, reason=\"{}\", changed=false)\n{}", $id, $reason, $new)
        }
    }};
}

/// Comprehensive transformation tracker that logs all changes
pub struct TransformationTracker {
    pub transformations: Vec<TransformationRecord>,
    pub audit_log: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TransformationRecord {
    pub id: String,
    pub description: String,
    pub file_path: String,
    pub applied: bool,
    pub before_hash: u64,
    pub after_hash: u64,
}

impl TransformationTracker {
    pub fn new() -> Self {
        Self {
            transformations: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    pub fn track<F>(&mut self, id: &str, desc: &str, file_path: &str, content: &str, transform_fn: F) -> String 
    where 
        F: FnOnce(&str) -> String,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let before_hash = {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            hasher.finish()
        };

        let transformed = transform_fn(content);
        
        let after_hash = {
            let mut hasher = DefaultHasher::new();
            transformed.hash(&mut hasher);
            hasher.finish()
        };

        let applied = before_hash != after_hash;
        
        let record = TransformationRecord {
            id: id.to_string(),
            description: desc.to_string(),
            file_path: file_path.to_string(),
            applied,
            before_hash,
            after_hash,
        };

        self.transformations.push(record);
        
        let log_entry = format!("{}[{}] {} on {} - {}", 
            if applied { "✅ " } else { "⏭️ " },
            id, 
            desc, 
            file_path,
            if applied { "APPLIED" } else { "SKIPPED" }
        );
        self.audit_log.push(log_entry);

        // Add tracking comment to the transformed code
        if applied {
            format!("// @transform(id=\"{}\", desc=\"{}\", file=\"{}\", hash_before={}, hash_after={}, applied=true)\n{}", 
                id, desc, file_path, before_hash, after_hash, transformed)
        } else {
            format!("// @transform(id=\"{}\", desc=\"{}\", file=\"{}\", hash_before={}, hash_after={}, applied=false)\n{}", 
                id, desc, file_path, before_hash, after_hash, transformed)
        }
    }

    pub fn save_audit_log(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        use std::fs;
        let log_content = self.audit_log.join("\n");
        fs::write(path, log_content)?;
        Ok(())
    }

    pub fn get_summary(&self) -> String {
        let total = self.transformations.len();
        let applied = self.transformations.iter().filter(|t| t.applied).count();
        let skipped = total - applied;
        
        format!("📊 Transformation Summary: {} total, {} applied, {} skipped", total, applied, skipped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_transform_macro() {
        let content = "fn test() {}";
        let result = track_transform!(content, "T001", "add feature flag", |c: &str| {
            format!("#![feature(test)]\n{}", c)
        });
        
        assert!(result.contains("@transform(id=T001"));
        assert!(result.contains("applied=true"));
        assert!(result.contains("#![feature(test)]"));
    }

    #[test]
    fn test_transformation_tracker() {
        let mut tracker = TransformationTracker::new();
        
        let content = "fn test() {}";
        let result = tracker.track("T001", "add feature", "test.rs", content, |c| {
            format!("#![feature(test)]\n{}", c)
        });
        
        assert!(result.contains("@transform"));
        assert_eq!(tracker.transformations.len(), 1);
        assert!(tracker.transformations[0].applied);
    }

    #[test]
    fn test_no_change_tracking() {
        let mut tracker = TransformationTracker::new();
        
        let content = "fn test() {}";
        let result = tracker.track("T001", "no-op", "test.rs", content, |c| c.to_string());
        
        assert!(result.contains("applied=false"));
        assert!(!tracker.transformations[0].applied);
    }
}
