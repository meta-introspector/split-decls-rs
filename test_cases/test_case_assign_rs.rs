// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/src/ops/assign.rs
// Error: expected square brackets
// Problematic line: line 11


// Arithmetic

macro_rules! assign_ops {
    ($(impl<T, U, const N: usize> $assignTrait:ident<U> for Simd<T, N>
        where
            Self: $trait:ident,
