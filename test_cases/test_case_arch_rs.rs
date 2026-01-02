// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/fortanix_sgx/arch.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::mem::MaybeUninit;

/// Wrapper struct to force 16-byte alignment.
#[repr(align(16))]
#[unstable(feature = "sgx_platform", issue = "56975")]
pub struct Align16<T>(pub T);
