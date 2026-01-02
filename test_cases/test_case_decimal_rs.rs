// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/decimal.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::num::dec2flt::float::RawFloat;
use crate::num::dec2flt::fpu::set_precision;

const INT_POW10: [u64; 16] = [
    1,
    10,
    100,
