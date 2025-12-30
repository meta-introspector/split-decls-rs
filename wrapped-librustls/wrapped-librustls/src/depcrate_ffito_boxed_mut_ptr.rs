// Generated macro for to_boxed_mut_ptr (function)
macro_rules! Depcrate_ffito_boxed_mut_ptr {
() => {
// Module: crate::ffi
// Provides: {"to_boxed_mut_ptr"}
// Dependencies: {}
# [doc = " Converts a [`Castable`]'s underlying [`Castable::RustType`] to a mutable pointer"] # [doc = " to a `Box` over the rust type."] # [doc = ""] # [doc = " Can only be used when the `Castable` has specified a cast type equal to [`OwnershipBox`]."] pub (crate) fn to_boxed_mut_ptr < C > (src : C :: RustType) -> * mut C where C : Castable < Ownership = OwnershipBox > , { Box :: into_raw (Box :: new (src)) as * mut _ }
};
}
