// SRC: ../rust/library/portable-simd/crates/core_simd/tests/usize_ops.rs
#![feature(portable_simd)]

#[macro_use]
mod ops_macros;
impl_unsigned_tests! { usize }
