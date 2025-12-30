// Generated macro for line_cb_c (function)
macro_rules! Depcrate_diffline_cb_c {
() => {
// Module: crate::diff
// Provides: {"line_cb_c"}
// Dependencies: {}
pub extern "C" fn line_cb_c (delta : * const raw :: git_diff_delta , hunk : * const raw :: git_diff_hunk , line : * const raw :: git_diff_line , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let hunk = Binding :: from_raw_opt (hunk) ; let line = Binding :: from_raw (line) ; let r = panic :: wrap (| | { let cbs = data as * mut DiffCallbacks < '_ , '_ , '_ , '_ , '_ , '_ , '_ , '_ > ; match (* cbs) . line { Some (ref mut cb) => cb (delta , hunk , line) , None => false , } }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
};
}
