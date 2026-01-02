// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_span/src/source_map.rs
// Error: expected square brackets
// Problematic line: line 23


use crate::*;

#[cfg(test)]
mod tests;

/// Returns the span itself if it doesn't come from a macro expansion,
