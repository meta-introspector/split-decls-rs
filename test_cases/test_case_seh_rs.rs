// MINIMAL TEST CASE for parsing failure in: ../rust/library/panic_unwind/src/seh.rs
// Error: expected square brackets
// Problematic line: line 55

use core::mem::ManuallyDrop;

// NOTE(nbdd0121): The `canary` field is part of stable ABI.
#[repr(C)]
struct Exception {
    // See `gcc.rs` on why this is present. We already have a static here so just use it.
    canary: *const _TypeDescriptor,
