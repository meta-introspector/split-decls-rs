// Generated macro for set_arc_mut_ptr (function)
macro_rules! Depcrate_ffiset_arc_mut_ptr {
() => {
// Module: crate::ffi
// Provides: {"set_arc_mut_ptr"}
// Dependencies: {}
# [doc = " Converts a [`Castable`]'s underlying [`Castable::RustType`] to a const pointer"] # [doc = " to an `Arc` over the rust type and sets the `dst` out pointer to the resulting const `Arc`"] # [doc = " pointer. See [`to_arc_const_ptr`] for more information."] # [doc = ""] # [doc = " ## Unsafety:"] # [doc = ""] # [doc = " `dst` must not be `NULL`."] pub (crate) fn set_arc_mut_ptr < C > (dst : & mut * const C , src : C :: RustType) where C : Castable < Ownership = OwnershipArc > , { * dst = to_arc_const_ptr (src) ; }
};
}
