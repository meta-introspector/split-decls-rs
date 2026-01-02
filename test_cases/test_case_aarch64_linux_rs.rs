// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/compiler-builtins/src/aarch64_linux.rs
// Error: expected square brackets
// Problematic line: line 26


use core::sync::atomic::{AtomicU8, Ordering};

/// non-zero if the host supports LSE atomics.
static HAVE_LSE_ATOMICS: AtomicU8 = AtomicU8::new(0);

intrinsics! {
