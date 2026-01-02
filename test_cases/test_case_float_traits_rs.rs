// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/support/float_traits.rs
// Error: expected square brackets
// Problematic line: line 7


use super::int_traits::{CastFrom, Int, MinInt};

/// Trait for some basic operations on floats
// #[allow(dead_code)]
#[allow(dead_code)] // Some constants are only used with tests
pub trait Float:
