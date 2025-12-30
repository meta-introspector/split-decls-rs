// Generated macro for macro_45 (macro)
macro_rules! Depcrate_ruremacro_45 {
() => {
// Module: crate::rure
// Provides: {"macro_45"}
// Dependencies: {}
ffi_fn ! { fn rure_free (re : * const Regex) { unsafe { drop (Box :: from_raw (re as * mut Regex)) ; } } }
};
}
