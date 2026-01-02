// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/futex.rs
// Error: expected square brackets
// Problematic line: line 1

#![cfg(any(
    target_os = "linux",
    target_os = "android",
    all(target_os = "emscripten", target_feature = "atomics"),
