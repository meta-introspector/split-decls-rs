// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/stdio/teeos.rs
// Error: expected square brackets
// Problematic line: line 3

#![deny(unsafe_op_in_unsafe_fn)]

#[expect(dead_code)]
#[path = "unsupported.rs"]
mod unsupported_stdio;

