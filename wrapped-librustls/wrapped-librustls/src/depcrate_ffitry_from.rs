// Generated macro for try_from (function)
macro_rules! Depcrate_ffitry_from {
() => {
// Module: crate::ffi
// Provides: {"try_from"}
// Dependencies: {}
# [doc = " Converts a const pointer to a [`Castable`] to an optional ref to the underlying"] # [doc = " [`Castable::RustType`]. See [`cast_const_ptr`] for more information."] # [doc = ""] # [doc = " Does nothing, returning `None` when passed `NULL`. Can be used with `Castable`'s that"] # [doc = " specify a cast type of [`OwnershipArc`] as well as `Castable`'s that specify"] # [doc = " a cast type of [`OwnershipBox`]."] pub (crate) fn try_from < 'a , C , O > (from : * const C) -> Option < & 'a C :: RustType > where C : Castable < Ownership = O > , { unsafe { cast_const_ptr (from) . as_ref () } }
};
}
