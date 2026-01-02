// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/f32.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::panic::const_assert;
use crate::{cfg_select, intrinsics, mem};

/// The radix or base of the internal representation of `f32`.
/// Use [`f32::RADIX`] instead.
///
/// # Examples
