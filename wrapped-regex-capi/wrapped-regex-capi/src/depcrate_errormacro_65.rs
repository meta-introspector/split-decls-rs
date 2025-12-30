// Generated macro for macro_65 (macro)
macro_rules! Depcrate_errormacro_65 {
() => {
// Module: crate::error
// Provides: {"macro_65"}
// Dependencies: {}
ffi_fn ! { fn rure_error_message (err : * mut Error) -> * const c_char { let err = unsafe { & mut * err } ; let cmsg = match CString :: new (format ! ("{}" , err)) { Ok (msg) => msg , Err (err) => { let nul = err . nul_position () ; let msg = err . into_vec () ; CString :: new (msg [0 .. nul] . to_owned ()) . unwrap () } } ; let p = cmsg . as_ptr () ; err . message = Some (cmsg) ; p } }
};
}
