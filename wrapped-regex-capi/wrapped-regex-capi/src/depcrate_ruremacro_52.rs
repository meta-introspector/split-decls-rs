// Generated macro for macro_52 (macro)
macro_rules! Depcrate_ruremacro_52 {
() => {
// Module: crate::rure
// Provides: {"macro_52"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_capture_names_free (it : * mut IterCaptureNames) { unsafe { let it = & mut * it ; while let Some (ptr) = it . name_ptrs . pop () { drop (CString :: from_raw (ptr)) ; } drop (Box :: from_raw (it)) ; } } }
};
}
