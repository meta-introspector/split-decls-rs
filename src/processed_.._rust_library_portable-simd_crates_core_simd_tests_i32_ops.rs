// SRC: ../rust/library/portable-simd/crates/core_simd/tests/i32_ops.rs
#![feature(portable_simd)]

#[macro_use]
mod ops_macros;
impl_signed_tests! { i32 }
