// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/fminimum.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::support::Float;

#[inline]
pub fn fminimum<F: Float>(x: F, y: F) -> F {
    let res = if x.is_nan() {
        x
