// Generated macro for try_from_mut_mut (function)
macro_rules! Depcrate_ffitry_from_mut_mut {
() => {
// Module: crate::ffi
// Provides: {"try_from_mut_mut"}
// Dependencies: {}
# [doc = " Converts a mutable pointer to a mutable pointer to a [`Castable`] to an optional mutable ref to"] # [doc = " the mutable pointer to the  [`Castable::RustType`]."] # [doc = ""] # [doc = " Does nothing, returning `None`, when passed `NULL`."] pub (crate) fn try_from_mut_mut < 'a , C : Castable > (from : * mut * mut C) -> Option < & 'a mut * mut C > { match from . is_null () { true => None , false => unsafe { Some (& mut * from) } , } }
};
}
