// Generated macro for callback (function)
macro_rules! Depcrate_stats_printcallback {
() => {
// Module: crate::stats_print
// Provides: {"callback"}
// Dependencies: {}
extern "C" fn callback < W > (opaque : * mut c_void , buf : * const c_char) where W : Write , { unsafe { let state = & mut * (opaque as * mut State < W >) ; if state . error . is_err () || state . panic . is_err () { return ; } let buf = CStr :: from_ptr (buf) ; match panic :: catch_unwind (AssertUnwindSafe (| | { state . writer . write_all (buf . to_bytes ()) })) { Ok (Ok (_)) => { } Ok (Err (e)) => state . error = Err (e) , Err (e) => state . panic = Err (e) , } } }
};
}
