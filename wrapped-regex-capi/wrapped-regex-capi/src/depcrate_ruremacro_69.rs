// Generated macro for macro_69 (macro)
macro_rules! Depcrate_ruremacro_69 {
() => {
// Module: crate::rure
// Provides: {"macro_69"}
// Dependencies: {}
ffi_fn ! { fn rure_set_matches (re : * const RegexSet , haystack : * const u8 , len : size_t , start : size_t , matches : * mut bool) -> bool { let re = unsafe { &* re } ; let mut matches = unsafe { slice :: from_raw_parts_mut (matches , re . len ()) } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; for item in matches . iter_mut () { * item = false ; } re . matches_read_at (& mut matches , haystack , start) } }
};
}
