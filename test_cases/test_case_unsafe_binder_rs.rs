// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/unsafe_binder.rs
// Error: expected square brackets
// Problematic line: line 3

//! Operators used to turn types into unsafe binders and back.

/// Unwrap an unsafe binder into its underlying type.
#[allow_internal_unstable(builtin_syntax)]
#[unstable(feature = "unsafe_binders", issue = "130516")]
pub macro unwrap_binder {
