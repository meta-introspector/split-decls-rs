// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/arch/i586.rs
// Error: expected square brackets
// Problematic line: line 10

//! See https://github.com/rust-lang/compiler-builtins/pull/976 for discussion on why these
//! functions are implemented in this way.

pub fn ceil(mut x: f64) -> f64 {
    unsafe {
        core::arch::asm!(
            "fld qword ptr [{x}]",
