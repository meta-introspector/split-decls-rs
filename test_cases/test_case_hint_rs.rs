// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/hint.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::mem::MaybeUninit;
use crate::{intrinsics, ub_checks};

/// Informs the compiler that the site which is calling this function is not
/// reachable, possibly enabling further optimizations.
///
/// # Safety
