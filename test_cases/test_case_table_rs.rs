// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/table.rs
// Error: expected square brackets
// Problematic line: line 11


pub(super) const SMALLEST_POWER_OF_FIVE: i32 = -342;
pub(super) const LARGEST_POWER_OF_FIVE: i32 = 308;
pub(super) const N_POWERS_OF_FIVE: usize =
    (LARGEST_POWER_OF_FIVE - SMALLEST_POWER_OF_FIVE + 1) as usize;

// Use static to avoid long compile times: Rust compiler errors
