// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/src/simd/ptr.rs
// Error: expected square brackets
// Problematic line: line 6

mod const_ptr;
mod mut_ptr;

mod sealed {
    pub trait Sealed {}
}

