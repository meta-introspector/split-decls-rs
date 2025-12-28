macro_rules! errmsg_to_string {
    () => {
        unsafe fn errmsg_to_string (errmsg : * const c_char) -> String { CStr :: from_ptr (errmsg) . to_string_lossy () . into_owned () }
    };
}

errmsg_to_string!()