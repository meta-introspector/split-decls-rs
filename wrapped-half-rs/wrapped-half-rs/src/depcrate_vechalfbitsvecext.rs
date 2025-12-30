// Generated macro for HalfBitsVecExt (trait)
macro_rules! Depcrate_vecHalfBitsVecExt {
() => {
// Module: crate::vec
// Provides: {"HalfBitsVecExt"}
// Dependencies: {}
# [doc = " Extensions to [`Vec<u16>`] to support reinterpret operations."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented outside of this crate."] pub trait HalfBitsVecExt : private :: SealedHalfBitsVec { # [doc = " Reinterprets a vector of [`u16`] bits as a vector of [`struct@f16`] or [`bf16`] numbers."] # [doc = ""] # [doc = " `H` is the type to cast to, and must be either the [`struct@f16`] or [`bf16`] type."] # [doc = ""] # [doc = " This is a zero-copy operation. The reinterpreted vector has the same memory location as"] # [doc = " `self`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use half::prelude::*;"] # [doc = " let int_buffer = vec![f16::from_f32(1.).to_bits(), f16::from_f32(2.).to_bits(), f16::from_f32(3.).to_bits()];"] # [doc = " let float_buffer = int_buffer.reinterpret_into::<f16>();"] # [doc = ""] # [doc = " assert_eq!(float_buffer, [f16::from_f32(1.), f16::from_f32(2.), f16::from_f32(3.)]);"] # [doc = " ```"] # [must_use] fn reinterpret_into < H > (self) -> Vec < H > where H : crate :: private :: SealedHalf ; }
};
}
