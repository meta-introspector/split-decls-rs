// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/example/float-minmax-pass.rs
// Error: expected square brackets
// Problematic line: line 10

#![feature(repr_simd, core_intrinsics)]
#![allow(internal_features, non_camel_case_types)]

#[repr(simd)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct f32x4(pub [f32; 4]);

