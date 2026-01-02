// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/fmax.rs
// Error: expected square brackets
// Problematic line: line 19


use crate::support::Float;

#[inline]
pub fn fmax<F: Float>(x: F, y: F) -> F {
    let res = if x.is_nan() || x < y { y } else { x };
    res.canonicalize()
