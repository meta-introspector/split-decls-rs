// Generated macro for macro_60 (macro)
macro_rules! Depcrate_ruremacro_60 {
() => {
// Module: crate::rure
// Provides: {"macro_60"}
// Dependencies: {}
ffi_fn ! { fn rure_captures_at (captures : * const Captures , i : size_t , match_info : * mut rure_match ,) -> bool { let locs = unsafe { & (* captures) . 0 } ; match locs . get (i) { Some ((start , end)) => { if ! match_info . is_null () { unsafe { (* match_info) . start = start ; (* match_info) . end = end ; } } true } _ => false } } }
};
}
