// Generated macro for impl_227 (impl)
macro_rules! Depcrate_repr_numimpl_227 {
() => {
// Module: crate::repr::num
// Provides: {"impl_227"}
// Dependencies: {}
# [doc = " For 128-bit integer types we use the [`itoa`] crate because writing into a buffer, and then"] # [doc = " copying the amount of characters we've written, is faster than determining the number of"] # [doc = " characters and then writing."] impl IntoRepr for u128 { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { let mut buffer = itoa :: Buffer :: new () ; Ok (Repr :: new (buffer . format (self)) ?) } }
};
}
