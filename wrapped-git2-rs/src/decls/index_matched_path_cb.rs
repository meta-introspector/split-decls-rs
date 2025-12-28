macro_rules! deps {
    () => {
        IndexMatchedPath!();
    };
}

macro_rules! index_matched_path_cb {
    () => {
        deps!();
        extern "C" fn index_matched_path_cb (path : * const c_char , matched_pathspec : * const c_char , payload : * mut c_void ,) -> c_int { unsafe { let path = CStr :: from_ptr (path) . to_bytes () ; let matched_pathspec = CStr :: from_ptr (matched_pathspec) . to_bytes () ; panic :: wrap (| | { let payload = payload as * mut & mut IndexMatchedPath < '_ > ; (* payload) (util :: bytes2path (path) , matched_pathspec) as c_int }) . unwrap_or (- 1) } }
    };
}

index_matched_path_cb!();