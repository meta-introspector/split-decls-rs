use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait for computing the semantic hash (Gödel number) and Monster Group factors for a Declaration.
pub trait SemanticHasher {
    /// Computes the semantic hash and Monster Group factors for a given Declaration.
    /// Returns a new Declaration with the computed fields populated.
    fn compute_semantic_hash(&self, declaration: Declaration) -> Declaration;
}
