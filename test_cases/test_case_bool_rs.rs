// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/bool.rs
// Error: expected square brackets
// Problematic line: line 3

//! impl bool {}

impl bool {
    /// Returns `Some(t)` if the `bool` is [`true`](../std/keyword.true.html),
    /// or `None` otherwise.
    ///
