// Generated macro for binary_cb_c (function)
macro_rules! Depcrate_diffbinary_cb_c {
() => {
// Module: crate::diff
// Provides: {"binary_cb_c"}
// Dependencies: {}
pub extern "C" fn binary_cb_c (delta : * const raw :: git_diff_delta , binary : * const raw :: git_diff_binary , data : * mut c_void ,) -> c_int { unsafe { let delta = Binding :: from_raw (delta as * mut _) ; let binary = Binding :: from_raw (binary) ; let r = panic :: wrap (| | { let cbs = data as * mut DiffCallbacks < '_ , '_ , '_ , '_ , '_ , '_ , '_ , '_ > ; match (* cbs) . binary { Some (ref mut cb) => cb (delta , binary) , None => false , } }) ; if r == Some (true) { raw :: GIT_OK } else { raw :: GIT_EUSER } } }
};
}
