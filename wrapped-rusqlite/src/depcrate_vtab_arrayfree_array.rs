// Generated macro for free_array (function)
macro_rules! Depcrate_vtab_arrayfree_array {
() => {
// Module: crate::vtab::array
// Provides: {"free_array"}
// Dependencies: {}
pub (crate) unsafe extern "C" fn free_array (p : * mut c_void) { Rc :: decrement_strong_count (p as * const Vec < Value >) ; }
};
}
