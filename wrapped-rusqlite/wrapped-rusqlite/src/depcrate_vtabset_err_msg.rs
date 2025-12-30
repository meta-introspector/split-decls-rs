// Generated macro for set_err_msg (function)
macro_rules! Depcrate_vtabset_err_msg {
() => {
// Module: crate::vtab
// Provides: {"set_err_msg"}
// Dependencies: {}
# [doc = " Virtual tables methods can set an error message by assigning a string to"] # [doc = " `zErrMsg`."] # [cold] unsafe fn set_err_msg (vtab : * mut sqlite3_vtab , err_msg : & str) { if ! (* vtab) . zErrMsg . is_null () { ffi :: sqlite3_free ((* vtab) . zErrMsg . cast :: < c_void > ()) ; } (* vtab) . zErrMsg = alloc (err_msg) ; }
};
}
