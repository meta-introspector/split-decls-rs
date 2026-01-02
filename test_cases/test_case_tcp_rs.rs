// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/net/tcp.rs
// Error: expected square brackets
// Problematic line: line 3

#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(all(
    test,
    not(any(
        target_os = "emscripten",
