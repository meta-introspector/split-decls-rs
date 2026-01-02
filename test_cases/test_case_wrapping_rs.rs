// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/wrapping.rs
// Error: expected square brackets
// Problematic line: line 4

//! Definitions of `Wrapping<T>`.

use crate::fmt;
use crate::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
