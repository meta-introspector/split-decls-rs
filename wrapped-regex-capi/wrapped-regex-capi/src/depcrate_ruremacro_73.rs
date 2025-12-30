// Generated macro for macro_73 (macro)
macro_rules! Depcrate_ruremacro_73 {
() => {
// Module: crate::rure
// Provides: {"macro_73"}
// Dependencies: {}
ffi_fn ! { fn rure_cstring_free (s : * mut c_char) { unsafe { drop (CString :: from_raw (s)) ; } } }
};
}
