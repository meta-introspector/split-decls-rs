// SRC: ../rust/library/portable-simd/crates/core_simd/src/simd/num.rs
//! Traits for vectors with numeric elements.

mod float;
mod int;
mod uint;

mod sealed {
    pub trait Sealed {}
}

pub use float::*;
pub use int::*;
pub use uint::*;
