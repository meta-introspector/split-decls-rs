// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/eflags.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::arch::asm;

/// Reads EFLAGS.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=__readeflags)
#[cfg(target_arch = "x86")]
