// Generated macro for set_err_io (function)
macro_rules! Depcrate_transportset_err_io {
() => {
// Module: crate::transport
// Provides: {"set_err_io"}
// Dependencies: {}
unsafe fn set_err_io (e : & io :: Error) { let s = CString :: new (e . to_string ()) . unwrap () ; raw :: git_error_set_str (raw :: GIT_ERROR_NET as c_int , s . as_ptr ()) ; }
};
}
