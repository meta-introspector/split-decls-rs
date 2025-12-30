// Generated macro for macro_38 (macro)
macro_rules! Depcrate_ruremacro_38 {
() => {
// Module: crate::rure
// Provides: {"macro_38"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_capture_names_new (re : * const Regex ,) -> * mut IterCaptureNames { let re = unsafe { &* re } ; Box :: into_raw (Box :: new (IterCaptureNames { capture_names : re . re . capture_names () , name_ptrs : Vec :: new () , })) } }
};
}
