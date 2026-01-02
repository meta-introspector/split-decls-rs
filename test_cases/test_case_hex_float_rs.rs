// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/support/hex_float.rs
// Error: expected square brackets
// Problematic line: line 5


use super::{Round, Status, f32_from_bits, f64_from_bits};

/// Construct a 16-bit float from hex float representation (C-style)
#[cfg(f16_enabled)]
pub const fn hf16(s: &str) -> f16 {
    match parse_hex_exact(s, 16, 10) {
