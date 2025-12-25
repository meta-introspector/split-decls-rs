use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A struct to track strings from input to output
/// Ensures no strings are lost during processing
#[derive(Debug, Default)]
pub struct StringTracker {
    // Maps string hash to its source location and content
    input_strings: HashMap<u64, StringRecord>,
    // Maps string hash to its output location and content
    output_strings: HashMap<u64, StringRecord>,
    // Tracks which input strings have been accounted for in output
    accounted_for: HashSet<u64>,
}

/// Records information about a string
#[derive(Debug, Clone)]
pub struct StringRecord {
    pub content: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
}

impl StringTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an input string to track
    pub fn track_input(&mut self, content: &str, file: &str, line: usize, column: usize, byte_offset: usize) {
        let hash = hash_string(content);
        self.input_strings.insert(hash, StringRecord {
            content: content.to_string(),
            file: file.to_string(),
            line,
            column,
            byte_offset,
            byte_length: content.len(),
        });
    }

    /// Add an output string to track
    pub fn track_output(&mut self, content: &str, file: &str, line: usize, column: usize, byte_offset: usize) {
        let hash = hash_string(content);
        self.output_strings.insert(hash, StringRecord {
            content: content.to_string(),
            file: file.to_string(),
            line,
            column,
            byte_offset,
            byte_length: content.len(),
        });
        
        // Mark as accounted for if this string existed in input
        if self.input_strings.contains_key(&hash) {
            self.accounted_for.insert(hash);
        }
    }

    /// Verify all input strings are accounted for in output
    pub fn verify_complete(&self) -> Result<(), Vec<StringRecord>> {
        let mut missing = Vec::new();
        
        for (hash, record) in &self.input_strings {
            if !self.accounted_for.contains(hash) {
                // Check if there's a substring match
                let mut found = false;
                for output_record in self.output_strings.values() {
                    if output_record.content.contains(&record.content) {
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    missing.push(record.clone());
                }
            }
        }
        
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }

    /// Get summary statistics
    pub fn summary(&self) -> String {
        format!(
            "StringTracker summary: {} input strings, {} output strings, {} accounted for",
            self.input_strings.len(),
            self.output_strings.len(),
            self.accounted_for.len()
        )
    }
}

/// Hash a string for tracking
fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Case-insensitive substring search
pub fn contains_ignoring_case(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_tracker() {
        let mut tracker = StringTracker::new();
        
        tracker.track_input("hello world", "src/main.rs", 10, 5, 100);
        tracker.track_output("hello world", "output/main.rs", 5, 3, 50);
        
        assert!(tracker.verify_complete().is_ok());
    }

    #[test]
    fn test_missing_string() {
        let mut tracker = StringTracker::new();
        
        tracker.track_input("hello world", "src/main.rs", 10, 5, 100);
        tracker.track_output("goodbye world", "output/main.rs", 5, 3, 50);
        
        assert!(tracker.verify_complete().is_err());
    }
}