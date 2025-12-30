// Generated macro for macro_47 (macro)
macro_rules! Depcrate_ruremacro_47 {
() => {
// Module: crate::rure
// Provides: {"macro_47"}
// Dependencies: {}
ffi_fn ! { fn rure_find (re : * const Regex , haystack : * const u8 , len : size_t , start : size_t , match_info : * mut rure_match ,) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; re . find_at (haystack , start) . map (| m | unsafe { if ! match_info . is_null () { (* match_info) . start = m . start () ; (* match_info) . end = m . end () ; } }) . is_some () } }
};
}
