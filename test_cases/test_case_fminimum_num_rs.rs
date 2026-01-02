// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/fminimum_num.rs
// Error: expected square brackets
// Problematic line: line 16


use crate::support::Float;

#[inline]
pub fn fminimum_num<F: Float>(x: F, y: F) -> F {
    let res = if x > y || x.is_nan() {
        y
