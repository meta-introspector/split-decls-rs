// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/int_log10.rs
// Error: expected square brackets
// Problematic line: line 5

//! that someone has already checked that the value is strictly positive.

// 0 < val <= u8::MAX
#[inline]
pub(super) const fn u8(val: u8) -> u32 {
    let val = val as u32;

