// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/support/big.rs
// Error: expected square brackets
// Problematic line: line 3

//! Integers used for wide operations, larger than `u128`.

#[cfg(test)]
mod tests;

use core::ops;
