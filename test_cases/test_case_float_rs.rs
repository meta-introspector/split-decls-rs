// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/float.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::num::FpCategory;
use crate::ops::{self, Add, Div, Mul, Neg};

/// Lossy `as` casting between two types.
pub trait CastInto<T: Copy>: Copy {
    fn cast(self) -> T;
}
