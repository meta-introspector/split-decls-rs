// Generated macro for cast_const_ptr (function)
macro_rules! Depcrate_fficast_const_ptr {
() => {
// Module: crate::ffi
// Provides: {"cast_const_ptr"}
// Dependencies: {}
# [doc = " Convert a const pointer to a [`Castable`] to a const pointer to its underlying"] # [doc = " [`Castable::RustType`]."] # [doc = ""] # [doc = " This can be used regardless of the [`Castable::Ownership`] as we can make const pointers for"] # [doc = " `Box`, `Arc` and ref types."] pub (crate) fn cast_const_ptr < C > (ptr : * const C) -> * const C :: RustType where C : Castable , { ptr as * const _ }
};
}
