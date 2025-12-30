// Generated macro for macro_68 (macro)
macro_rules! Depcrate_ruremacro_68 {
() => {
// Module: crate::rure
// Provides: {"macro_68"}
// Dependencies: {}
ffi_fn ! { fn rure_set_is_match (re : * const RegexSet , haystack : * const u8 , len : size_t , start : size_t) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; re . is_match_at (haystack , start) } }
};
}
