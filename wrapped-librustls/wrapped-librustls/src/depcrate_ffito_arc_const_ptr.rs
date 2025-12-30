// Generated macro for to_arc_const_ptr (function)
macro_rules! Depcrate_ffito_arc_const_ptr {
() => {
// Module: crate::ffi
// Provides: {"to_arc_const_ptr"}
// Dependencies: {}
# [doc = " Convert a [`Castable`]'s underlying [`Castable::RustType`] to a constant pointer"] # [doc = " to an `Arc` over the rust type. Can only be used when the `Castable` has specified a cast type"] # [doc = " equal to [`OwnershipArc`]."] pub (crate) fn to_arc_const_ptr < C > (src : C :: RustType) -> * const C where C : Castable < Ownership = OwnershipArc > , { Arc :: into_raw (Arc :: new (src)) as * const _ }
};
}
