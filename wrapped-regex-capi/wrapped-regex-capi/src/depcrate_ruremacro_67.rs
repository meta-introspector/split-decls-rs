// Generated macro for macro_67 (macro)
macro_rules! Depcrate_ruremacro_67 {
() => {
// Module: crate::rure
// Provides: {"macro_67"}
// Dependencies: {}
ffi_fn ! { fn rure_set_free (re : * const RegexSet) { unsafe { drop (Box :: from_raw (re as * mut RegexSet)) ; } } }
};
}
