// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/flt2dec/decoder.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::num::FpCategory;
use crate::num::dec2flt::float::RawFloat;

/// Decoded unsigned finite value, such that:
///
/// - The original value equals to `mant * 2^exp`.
///
