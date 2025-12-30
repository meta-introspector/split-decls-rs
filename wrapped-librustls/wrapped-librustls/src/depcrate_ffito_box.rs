// Generated macro for to_box (function)
macro_rules! Depcrate_ffito_box {
() => {
// Module: crate::ffi
// Provides: {"to_box"}
// Dependencies: {}
# [doc = " Convert a mutable pointer to a [`Castable`] to an optional `Box` over the underlying rust type."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed `NULL`. Can only be used when the `Castable` has"] # [doc = " specified a cast type equal to [`OwnershipBox`]."] # [doc = ""] # [doc = " ## Unsafety:"] # [doc = ""] # [doc = " If non-null, `ptr` must be a pointer that resulted from previously calling `Box::into_raw`,"] # [doc = " e.g. from using [`to_boxed_mut_ptr`]."] pub (crate) fn to_box < C > (ptr : * mut C) -> Option < Box < C :: RustType > > where C : Castable < Ownership = OwnershipBox > , { if ptr . is_null () { return None ; } let rs_typed = cast_mut_ptr (ptr) ; unsafe { Some (Box :: from_raw (rs_typed)) } }
};
}
