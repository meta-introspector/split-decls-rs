// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/ceil.rs
// Error: expected square brackets
// Problematic line: line 12


use crate::support::{Float, FpResult, Int, IntTy, MinInt, Status};

#[inline]
pub fn ceil<F: Float>(x: F) -> F {
    ceil_status(x).val
}
