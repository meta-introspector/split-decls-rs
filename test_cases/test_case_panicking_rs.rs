// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/panicking.rs
// Error: expected square brackets
// Problematic line: line 25

//! one function. The actual symbol is declared through the `#[panic_handler]` attribute.

#![allow(dead_code, missing_docs)]
#![unstable(
    feature = "panic_internals",
    reason = "internal details of the implementation of the `panic!` and related macros",
    issue = "none"
