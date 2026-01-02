// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/slow.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::num::dec2flt::decimal_seq::{DecimalSeq, parse_decimal_seq};
use crate::num::dec2flt::float::RawFloat;

/// Parse the significant digits and biased, binary exponent of a float.
///
/// This is a fallback algorithm that uses a big-integer representation
/// of the float, and therefore is considerably slower than faster
