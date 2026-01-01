// SRC: ../rust/library/portable-simd/crates/core_simd/tests/isize_ops.rs
#![feature(portable_simd)]

#[macro_use]
mod ops_macros;
impl_signed_tests! { isize }
