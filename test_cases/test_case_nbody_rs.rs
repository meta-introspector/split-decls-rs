// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/examples/nbody.rs
// Error: expected square brackets
// Problematic line: line 5

#![allow(clippy::excessive_precision)]
extern crate std_float;

/// Benchmarks game nbody code
/// Taken from the `packed_simd` crate
/// Run this benchmark with `cargo test --example nbody`
mod nbody {
