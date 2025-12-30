// Generated macro for macro_39 (macro)
macro_rules! Depcrate_ruremacro_39 {
() => {
// Module: crate::rure
// Provides: {"macro_39"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_capture_names_free (it : * mut IterCaptureNames) { unsafe { let it = & mut * it ; while let Some (ptr) = it . name_ptrs . pop () { CString :: from_raw (ptr) ; } Box :: from_raw (it) ; } } }
};
}
