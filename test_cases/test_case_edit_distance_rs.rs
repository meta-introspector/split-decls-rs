// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_span/src/edit_distance.rs
// Error: expected square brackets
// Problematic line: line 16


use crate::Symbol;

#[cfg(test)]
mod tests;

/// Finds the [edit distance] between two strings.
