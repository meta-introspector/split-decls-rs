// Generated macro for raw_field (macro)
macro_rules! Depcrate_raw_fieldraw_field {
() => {
// Module: crate::raw_field
// Provides: {"raw_field"}
// Dependencies: {}
# [doc = " Computes a const raw pointer to the given field of the given base pointer"] # [doc = " to the given parent type."] # [doc = ""] # [doc = " The `base` pointer *must not* be dangling, but it *may* point to"] # [doc = " uninitialized memory."] # [macro_export (local_inner_macros)] macro_rules ! raw_field { ($ base : expr , $ parent : path , $ field : tt) => { { _memoffset__field_check ! ($ parent , $ field) ; let base = $ base ; # [allow (unused_unsafe)] unsafe { _memoffset__addr_of ! ((* (base as * const $ parent)) .$ field) } } } ; }
};
}
