// Generated macro for index_matched_path_cb (function)
macro_rules! Depcrate_indexindex_matched_path_cb {
() => {
// Module: crate::index
// Provides: {"index_matched_path_cb"}
// Dependencies: {}
extern "C" fn index_matched_path_cb (path : * const c_char , matched_pathspec : * const c_char , payload : * mut c_void ,) -> c_int { unsafe { let path = CStr :: from_ptr (path) . to_bytes () ; let matched_pathspec = CStr :: from_ptr (matched_pathspec) . to_bytes () ; panic :: wrap (| | { let payload = payload as * mut & mut IndexMatchedPath < '_ > ; (* payload) (util :: bytes2path (path) , matched_pathspec) as c_int }) . unwrap_or (- 1) } }
};
}
