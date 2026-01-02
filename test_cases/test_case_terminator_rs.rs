// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/terminator.rs
// Error: expected square brackets
// Problematic line: line 13


use super::*;

impl SwitchTargets {
    /// Creates switch targets from an iterator of values and target blocks.
    ///
    /// The iterator may be empty, in which case the `SwitchInt` instruction is equivalent to
