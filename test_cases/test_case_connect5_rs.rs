// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/examples/connect5.rs
// Error: expected square brackets
// Problematic line: line 42

use std::cmp;
use std::time::Instant;

#[cfg(target_arch = "x86")]
use core_arch::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core_arch::arch::x86_64::*;
