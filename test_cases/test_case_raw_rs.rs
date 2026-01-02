// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/sgx/abi/usercalls/raw.rs
// Error: expected square brackets
// Problematic line: line 3

#![allow(unused)]

#[unstable(feature = "sgx_platform", issue = "56975")]
pub use fortanix_sgx_abi::*;

use crate::num::NonZero;
