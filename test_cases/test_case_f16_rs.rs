// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/f16.rs
// Error: expected square brackets
// Problematic line: line 16


use crate::convert::FloatToInt;
use crate::num::FpCategory;
#[cfg(not(test))]
use crate::num::libm;
use crate::panic::const_assert;
use crate::{intrinsics, mem};
