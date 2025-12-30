// Generated macro for clone_arc (function)
macro_rules! Depcrate_fficlone_arc {
() => {
// Module: crate::ffi
// Provides: {"clone_arc"}
// Dependencies: {}
# [doc = " Given a const pointer to a [`Castable`] representing an `Arc`, clone the `Arc` and return"] # [doc = " the corresponding Rust type."] # [doc = ""] # [doc = " The caller still owns its copy of the `Arc`. In other words, the reference count of the"] # [doc = " `Arc` will be incremented by 1 by the end of this function."] # [doc = ""] # [doc = " To achieve that, we need to `mem::forget` the `Arc` we get back from `into_raw`, because"] # [doc = " `into_raw` _does_ take back ownership. If we called `into_raw` without `mem::forget`, at the"] # [doc = " end of the function that Arc would be dropped and the reference count would be decremented,"] # [doc = " potentially to 0, causing memory to be freed."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed a `NULL` pointer. Can only be used when the"] # [doc = " `Castable` has specified a cast type equal to [`OwnershipArc`]."] # [doc = ""] # [doc = " ## Unsafety:"] # [doc = ""] # [doc = " If non-null, `ptr` must be a pointer that resulted from previously calling `Arc::into_raw`,"] # [doc = " e.g. from using [`to_arc_const_ptr`]."] pub (crate) fn clone_arc < C > (ptr : * const C) -> Option < Arc < C :: RustType > > where C : Castable < Ownership = OwnershipArc > , { if ptr . is_null () { return None ; } let rs_typed = cast_const_ptr :: < C > (ptr) ; let r = unsafe { Arc :: from_raw (rs_typed) } ; let val = Arc :: clone (& r) ; mem :: forget (r) ; Some (val) }
};
}
