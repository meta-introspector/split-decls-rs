// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/fmaximum_num.rs
// Error: expected square brackets
// Problematic line: line 16


use crate::support::Float;

#[inline]
pub fn fmaximum_num<F: Float>(x: F, y: F) -> F {
    let res = if x > y || y.is_nan() {
        x
