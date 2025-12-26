use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a replacement of a `snippet`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Replacement {
    /// Code snippet that gets replaced.
    pub snippet: Snippet,
    /// The replacement of the snippet.
    pub replacement: String,
}
