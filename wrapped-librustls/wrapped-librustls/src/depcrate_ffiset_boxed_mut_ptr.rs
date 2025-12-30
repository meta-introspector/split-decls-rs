// Generated macro for set_boxed_mut_ptr (function)
macro_rules! Depcrate_ffiset_boxed_mut_ptr {
() => {
// Module: crate::ffi
// Provides: {"set_boxed_mut_ptr"}
// Dependencies: {}
# [doc = " Converts a [`Castable`]'s underlying [`Castable::RustType`] to a mutable pointer"] # [doc = " to a `Box` over the rust type and sets the `dst` out pointer to the resulting mutable `Box`"] # [doc = " pointer. See [`to_boxed_mut_ptr`] for more information."] pub (crate) fn set_boxed_mut_ptr < C > (dst : & mut * mut C , src : C :: RustType) where C : Castable < Ownership = OwnershipBox > , { * dst = to_boxed_mut_ptr (src) ; }
};
}
