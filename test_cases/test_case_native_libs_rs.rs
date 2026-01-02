// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/config/native_libs.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::config::UnstableOptions;
use crate::utils::NativeLib;

#[cfg(test)]
mod tests;

/// Parses all `-l` options.
