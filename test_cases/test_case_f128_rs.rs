// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/f128.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::panic::const_assert;
use crate::{intrinsics, mem};

/// Basic mathematical constants.
#[unstable(feature = "f128", issue = "116909")]
pub mod consts {
    // FIXME: replace with mathematical constants from cmath.
