// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/fmin.rs
// Error: expected square brackets
// Problematic line: line 19


use crate::support::Float;

#[inline]
pub fn fmin<F: Float>(x: F, y: F) -> F {
    let res = if y.is_nan() || x < y { x } else { y };
    res.canonicalize()
