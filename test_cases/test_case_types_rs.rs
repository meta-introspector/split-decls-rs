// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/types.rs
// Error: expected square brackets
// Problematic line: line 3

//! Platform dependent types.

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::Cow;
        use std::fmt;
