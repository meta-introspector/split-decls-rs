// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/f64.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::panic::const_assert;
use crate::{intrinsics, mem};

/// The radix or base of the internal representation of `f64`.
/// Use [`f64::RADIX`] instead.
///
/// # Examples
