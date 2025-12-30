// Generated macro for try_from_mut (function)
macro_rules! Depcrate_ffitry_from_mut {
() => {
// Module: crate::ffi
// Provides: {"try_from_mut"}
// Dependencies: {}
# [doc = " Converts a mutable pointer to a [`Castable`] to an optional ref to the underlying"] # [doc = " [`Castable::RustType`]. See [`cast_mut_ptr`] for more information."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed `NULL`. Can only be used when the `Castable` has"] # [doc = " specified a cast type equal to [`OwnershipBox`]."] pub (crate) fn try_from_mut < 'a , C > (from : * mut C) -> Option < & 'a mut C :: RustType > where C : Castable < Ownership = OwnershipBox > , { unsafe { cast_mut_ptr (from) . as_mut () } }
};
}
