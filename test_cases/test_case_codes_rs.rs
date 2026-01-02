// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/codes.rs
// Error: expected square brackets
// Problematic line: line 9


use std::fmt;

rustc_index::newtype_index! {
    #[max = 9999] // Because all error codes have four digits.
    #[orderable]
    #[encodable]
