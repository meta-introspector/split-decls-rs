// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/generic/sqrt.rs
// Error: expected square brackets
// Problematic line: line 44

//! Goldschmidt has the advantage over Newton-Raphson that `sqrt(x)` and `1/sqrt(x)` are
//! computed at the same time, i.e. there is no need to calculate `1/sqrt(x)` and invert it.

use crate::support::{
    CastFrom, CastInto, DInt, Float, FpResult, HInt, Int, IntTy, MinInt, Round, Status, cold_path,
};

