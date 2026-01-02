// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/config/externs.rs
// Error: expected square brackets
// Problematic line: line 10

use super::UnstableOptions;
use crate::EarlyDiagCtxt;

#[cfg(test)]
mod tests;

/// Represents the pieces of an `--extern` argument.
