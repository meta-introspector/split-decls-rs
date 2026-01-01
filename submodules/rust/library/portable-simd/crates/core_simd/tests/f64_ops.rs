// SRC: ../rust/library/portable-simd/crates/core_simd/tests/f64_ops.rs
#![feature(portable_simd)]

#[macro_use]
mod ops_macros;
impl_float_tests! { f64, i64 }
