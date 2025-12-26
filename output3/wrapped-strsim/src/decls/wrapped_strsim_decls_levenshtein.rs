use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one string into the other.
///
/// ```
/// use strsim::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    generic_levenshtein(&StringWrapper(a), &StringWrapper(b))
}
