// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/arch/wasm32.rs
// Error: expected square brackets
// Problematic line: line 4

//! Wasm has builtins for simple float operations. Use the unstable `core::arch` intrinsics which
//! are significantly faster than soft float operations.

pub fn ceil(x: f64) -> f64 {
    core::arch::wasm32::f64_ceil(x)
}

