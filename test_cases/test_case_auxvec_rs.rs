// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/os/linux/auxvec.rs
// Error: expected square brackets
// Problematic line: line 6


pub(crate) const AT_NULL: usize = 0;

/// Key to access the CPU Hardware capabilities bitfield.
pub(crate) const AT_HWCAP: usize = 16;
/// Key to access the CPU Hardware capabilities 2 bitfield.
#[cfg(any(
