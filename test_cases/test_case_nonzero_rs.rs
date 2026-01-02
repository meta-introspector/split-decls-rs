// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/nonzero.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::str::FromStr;
use crate::{fmt, intrinsics, ptr, ub_checks};

/// A marker trait for primitive types which can be zero.
///
/// This is an implementation detail for <code>[NonZero]\<T></code> which may disappear or be replaced at any time.
///
