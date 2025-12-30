// Generated macro for macro_33 (macro)
macro_rules! Depcrate_ruremacro_33 {
() => {
// Module: crate::rure
// Provides: {"macro_33"}
// Dependencies: {}
ffi_fn ! { fn rure_is_match (re : * const Regex , haystack : * const u8 , len : size_t , start : size_t ,) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; re . is_match_at (haystack , start) } }
};
}
