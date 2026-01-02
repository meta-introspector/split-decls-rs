// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/random.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::range::RangeFull;

/// A source of randomness.
#[unstable(feature = "random", issue = "130703")]
pub trait RandomSource {
    /// Fills `bytes` with random bytes.
