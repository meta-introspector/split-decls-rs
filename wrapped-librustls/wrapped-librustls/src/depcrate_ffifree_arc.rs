// Generated macro for free_arc (function)
macro_rules! Depcrate_ffifree_arc {
() => {
// Module: crate::ffi
// Provides: {"free_arc"}
// Dependencies: {}
# [doc = " Free a constant pointer to a [`Castable`]'s underlying [`Castable::RustType`] by"] # [doc = " reconstituting an `Arc` from the raw pointer and dropping it."] # [doc = ""] # [doc = " For types represented with an `Arc` on the Rust side, we offer a `_free()`"] # [doc = " method to the C side that decrements the refcount and ultimately drops"] # [doc = " the `Arc` if the refcount reaches 0. By contrast with `to_arc`, we call"] # [doc = " `Arc::from_raw` on the input pointer, but we _don't_ clone it, because we"] # [doc = " want the refcount to be lower by one when we reach the end of the function."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed `NULL`. Can only be used when the `Castable` has"] # [doc = " specified a cast type equal to [`OwnershipArc`]."] pub (crate) fn free_arc < C > (ptr : * const C) where C : Castable < Ownership = OwnershipArc > , { if ptr . is_null () { return ; } let rs_typed = cast_const_ptr (ptr) ; drop (unsafe { Arc :: from_raw (rs_typed) }) ; }
};
}
