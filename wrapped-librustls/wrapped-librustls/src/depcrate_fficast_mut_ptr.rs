// Generated macro for cast_mut_ptr (function)
macro_rules! Depcrate_fficast_mut_ptr {
() => {
// Module: crate::ffi
// Provides: {"cast_mut_ptr"}
// Dependencies: {}
# [doc = " Convert a mutable pointer to a [`Castable`] to a mutable pointer to its underlying"] # [doc = " [`Castable::RustType`]."] # [doc = ""] # [doc = " Can only be used when the `Castable` has specified a cast source equal to `BoxCastPtrMarker`."] pub (crate) fn cast_mut_ptr < C > (ptr : * mut C) -> * mut C :: RustType where C : Castable < Ownership = OwnershipBox > , { ptr as * mut _ }
};
}
