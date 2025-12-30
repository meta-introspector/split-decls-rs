// Generated macro for free_box (function)
macro_rules! Depcrate_ffifree_box {
() => {
// Module: crate::ffi
// Provides: {"free_box"}
// Dependencies: {}
# [doc = " Convert a mutable pointer to a [`Castable`] to an optional `Box` over the underlying"] # [doc = " [`Castable::RustType`], and immediately let it fall out of scope to be freed."] # [doc = ""] # [doc = " Can only be used when the `Castable` has specified a cast type equal to [`OwnershipBox`]."] # [doc = ""] # [doc = " ## Unsafety:"] # [doc = ""] # [doc = " If non-null, `ptr` must be a pointer that resulted from previously calling `Box::into_raw`,"] # [doc = " e.g. from using [`to_boxed_mut_ptr`]."] pub (crate) fn free_box < C > (ptr : * mut C) where C : Castable < Ownership = OwnershipBox > , { to_box (ptr) ; }
};
}
