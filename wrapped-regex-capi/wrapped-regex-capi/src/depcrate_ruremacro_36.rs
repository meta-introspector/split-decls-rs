// Generated macro for macro_36 (macro)
macro_rules! Depcrate_ruremacro_36 {
() => {
// Module: crate::rure
// Provides: {"macro_36"}
// Dependencies: {}
ffi_fn ! { fn rure_shortest_match (re : * const Regex , haystack : * const u8 , len : size_t , start : size_t , end : * mut usize ,) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; match re . shortest_match_at (haystack , start) { None => false , Some (i) => { if ! end . is_null () { unsafe { * end = i ; } } true } } } }
};
}
