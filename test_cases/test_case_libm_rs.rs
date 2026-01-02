// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/libm.rs
// Error: expected square brackets
// Problematic line: line 6


// SAFETY: These symbols have standard interfaces in C and are defined by `libm`, or are
// provided by `compiler-builtins` on unsupported platforms.
unsafe extern "C" {
    pub(crate) safe fn cbrt(n: f64) -> f64;
    pub(crate) safe fn cbrtf(n: f32) -> f32;
    pub(crate) safe fn fdim(a: f64, b: f64) -> f64;
