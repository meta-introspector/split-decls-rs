// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/lemire.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::num::dec2flt::common::BiasedFp;
use crate::num::dec2flt::float::RawFloat;
use crate::num::dec2flt::table::{
    LARGEST_POWER_OF_FIVE, POWER_OF_FIVE_128, SMALLEST_POWER_OF_FIVE,
};

