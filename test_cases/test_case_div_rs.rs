// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/compiler-builtins/src/float/div.rs
// Error: expected square brackets
// Problematic line: line 89

use crate::float::Float;
use crate::int::{CastFrom, CastInto, DInt, HInt, Int, MinInt};

fn div<F: Float>(a: F, b: F) -> F
where
    F::Int: CastInto<i32>,
    F::Int: From<HalfRep<F>>,
