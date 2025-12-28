macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! set_err_io {
    () => {
        deps!();
        unsafe fn set_err_io (e : & io :: Error) { let s = CString :: new (e . to_string ()) . unwrap () ; raw :: git_error_set_str (raw :: GIT_ERROR_NET as c_int , s . as_ptr ()) ; }
    };
}

set_err_io!();