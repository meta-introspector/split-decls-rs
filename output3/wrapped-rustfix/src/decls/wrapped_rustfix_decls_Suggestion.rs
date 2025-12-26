use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An error/warning and possible solutions for fixing it
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Suggestion {
    pub message: String,
    pub snippets: Vec<Snippet>,
    pub solutions: Vec<Solution>,
}
