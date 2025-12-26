use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Suggestions {
    /// Returns the underlying list of suggestions.
    pub fn unwrap_tag(self) -> Vec<CodeSuggestion> {
        match self {
            Suggestions::Enabled(suggestions) => suggestions,
            Suggestions::Sealed(suggestions) => suggestions.into_vec(),
            Suggestions::Disabled => Vec::new(),
        }
    }
}
