// Generated macro for try_from_ptr (function)
macro_rules! Depcrate_ffitry_from_ptr {
() => {
// Module: crate::ffi
// Provides: {"try_from_ptr"}
// Dependencies: {}
# [doc = " Converts a mut pointer to a const pointer to a [`Castable`] to an optional mut ref to the"] # [doc = " const pointer to the underlying [`Castable::RustType`]."] # [doc = ""] # [doc = " Does nothing, returning `None` when passed `NULL`."] pub (crate) fn try_from_ptr < 'a , C > (from : * mut * const C) -> Option < & 'a mut * const C > where C : Castable , { match from . is_null () { true => None , false => unsafe { Some (& mut * from) } , } }
};
}
