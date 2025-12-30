// Generated macro for macro_40 (macro)
macro_rules! Depcrate_ruremacro_40 {
() => {
// Module: crate::rure
// Provides: {"macro_40"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_capture_names_next (it : * mut IterCaptureNames , capture_name : * mut * mut c_char ,) -> bool { if capture_name . is_null () { return false ; } let it = unsafe { & mut * it } ; let cn = match it . capture_names . next () { None => return false , Some (val) => { let name = match val { None => "" , Some (name) => name } ; name } } ; unsafe { let cs = match CString :: new (cn . as_bytes ()) { Result :: Ok (val) => val , Result :: Err (err) => return false } ; let ptr = cs . into_raw () ; it . name_ptrs . push (ptr) ; * capture_name = ptr ; } true } }
};
}
