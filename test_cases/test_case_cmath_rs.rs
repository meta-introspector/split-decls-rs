// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/cmath.rs
// Error: expected square brackets
// Problematic line: line 5


// These symbols are all defined by `libm`,
// or by `compiler-builtins` on unsupported platforms.
unsafe extern "C" {
    pub safe fn acos(n: f64) -> f64;
    pub safe fn asin(n: f64) -> f64;
    pub safe fn atan(n: f64) -> f64;
