use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Like optimal string alignment, but substrings can be edited an unlimited
/// number of times, and the triangle inequality holds.
///
/// ```
/// use strsim::damerau_levenshtein;
///
/// assert_eq!(2, damerau_levenshtein("ab", "bca"));
/// ```
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    damerau_levenshtein_impl(a.chars(), a.chars().count(), b.chars(), b.chars().count())
}
