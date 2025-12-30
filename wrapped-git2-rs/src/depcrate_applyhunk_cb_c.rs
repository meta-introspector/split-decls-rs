// Generated macro for hunk_cb_c (function)
macro_rules! Depcrate_applyhunk_cb_c {
() => {
// Module: crate::apply
// Provides: {"hunk_cb_c"}
// Dependencies: {}
extern "C" fn hunk_cb_c (hunk : * const raw :: git_diff_hunk , data : * mut c_void) -> c_int { panic :: wrap (| | unsafe { let hunk = Binding :: from_raw_opt (hunk) ; let payload = & mut * (data as * mut ApplyOptions < '_ >) ; let callback = match payload . hunk_cb { Some (ref mut c) => c , None => return - 1 , } ; let apply = callback (hunk) ; if apply { 0 } else { 1 } }) . unwrap_or (- 1) }
};
}
