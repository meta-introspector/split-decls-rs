// Generated macro for try_box_from (function)
macro_rules! Depcrate_ffitry_box_from {
() => {
// Module: crate::ffi
// Provides: {"try_box_from"}
// Dependencies: {}
# [doc = " Convert a mutable pointer to a [`Castable`] to an optional `Box` over the underlying"] # [doc = " [`Castable::RustType`]."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed `NULL`. Can only be used with `Castable`'s that"] # [doc = " specify a cast type of [`OwnershipBox`]."] pub (crate) fn try_box_from < C > (from : * mut C) -> Option < Box < C :: RustType > > where C : Castable < Ownership = OwnershipBox > , { to_box (from) }
};
}
