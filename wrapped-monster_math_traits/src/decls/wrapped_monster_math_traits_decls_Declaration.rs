use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a parsed Rust code element (e.g., function, struct, enum, const).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Declaration {
    /// The kind of the declaration (e.g., "function", "struct", "enum").
    pub kind: String,
    /// The name of the declared item.
    pub name: String,
    /// The path within the source file where the item is declared.
    pub path: String,
    /// Placeholder for the semantic hash or Gödel number of the declaration.
    pub semantic_hash: Option<String>,
    /// Placeholder for the assigned Monster Group factors.
    pub monster_factors: Option<Vec<u32>>,
    /// Placeholder for the associated Bag of Words, used for exponent calculation.
    pub bag_of_words: Option<Vec<String>>,
    /// Placeholder for the associated 8D conceptual space coordinate.
    pub eight_d_coordinate: Option<Vec<f64>>,
    /// Dependencies identified for this declaration.
    pub deps: HashSet<String>,
    /// Whether the declaration is public.
    pub is_public: bool,
    /// Attributes applied to this declaration.
    pub attributes: HashSet<String>,
}
