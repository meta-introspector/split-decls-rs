// Generated macro for macro_48 (macro)
macro_rules! Depcrate_ruremacro_48 {
() => {
// Module: crate::rure
// Provides: {"macro_48"}
// Dependencies: {}
ffi_fn ! { fn rure_find_captures (re : * const Regex , haystack : * const u8 , len : size_t , start : size_t , captures : * mut Captures ,) -> bool { let re = unsafe { &* re } ; let haystack = unsafe { slice :: from_raw_parts (haystack , len) } ; let slots = unsafe { & mut (* captures) . 0 } ; re . captures_read_at (slots , haystack , start) . is_some () } }
};
}
