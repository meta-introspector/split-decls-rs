// Generated macro for macro_64 (macro)
macro_rules! Depcrate_ruremacro_64 {
() => {
// Module: crate::rure
// Provides: {"macro_64"}
// Dependencies: {}
ffi_fn ! { fn rure_options_size_limit (options : * mut Options , limit : size_t) { let options = unsafe { & mut * options } ; options . size_limit = limit ; } }
};
}
