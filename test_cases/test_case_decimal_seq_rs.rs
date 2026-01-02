// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/decimal_seq.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::num::dec2flt::common::{ByteSlice, is_8digits};

/// A decimal floating-point number, represented as a sequence of decimal digits.
#[derive(Clone, Debug, PartialEq)]
pub struct DecimalSeq {
    /// The number of significant digits in the decimal.
