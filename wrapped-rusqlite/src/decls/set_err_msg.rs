macro_rules! set_err_msg {
    () => {
        # [doc = " Virtual tables methods can set an error message by assigning a string to"] # [doc = " `zErrMsg`."] # [cold] unsafe fn set_err_msg (vtab : * mut sqlite3_vtab , err_msg : & str) { if ! (* vtab) . zErrMsg . is_null () { ffi :: sqlite3_free ((* vtab) . zErrMsg . cast :: < c_void > ()) ; } (* vtab) . zErrMsg = alloc (err_msg) ; }
    };
}

set_err_msg!();