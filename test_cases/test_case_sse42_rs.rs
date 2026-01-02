// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86_64/sse42.rs
// Error: expected square brackets
// Problematic line: line 3

//! `x86_64`'s Streaming SIMD Extensions 4.2 (SSE4.2)

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
