// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/shared/pivot.rs
// Error: expected square brackets
// Problematic line: line 8

// Recursively select a pseudomedian if above this threshold.
const PSEUDO_MEDIAN_REC_THRESHOLD: usize = 64;

/// Selects a pivot from `v`. Algorithm taken from glidesort by Orson Peters.
///
/// This chooses a pivot by sampling an adaptive amount of points, approximating
/// the quality of a median of sqrt(n) elements.
