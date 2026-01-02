// MINIMAL TEST CASE for parsing failure in: ../rust/library/panic_unwind/src/hermit.rs
// Error: expected square brackets
// Problematic line: line 8

use alloc::boxed::Box;
use core::any::Any;

unsafe extern "Rust" {
    // This is defined in std::rt
    #[rustc_std_internal_symbol]
    safe fn __rust_abort() -> !;
