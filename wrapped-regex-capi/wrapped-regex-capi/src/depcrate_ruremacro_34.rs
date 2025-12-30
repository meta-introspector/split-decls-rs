// Generated macro for macro_34 (macro)
macro_rules! Depcrate_ruremacro_34 {
() => {
// Module: crate::rure
// Provides: {"macro_34"}
// Dependencies: {}
ffi_fn ! { fn rure_find (re : * const Regex , haystack : * const u8 , len : size_t , start : size_t , match_info : * mut rure_match ,) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; re . find_at (haystack , start) . map (| (s , e) | unsafe { if ! match_info . is_null () { (* match_info) . start = s ; (* match_info) . end = e ; } }) . is_some () } }
};
}
