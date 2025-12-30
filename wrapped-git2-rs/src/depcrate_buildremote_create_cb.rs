// Generated macro for remote_create_cb (function)
macro_rules! Depcrate_buildremote_create_cb {
() => {
// Module: crate::build
// Provides: {"remote_create_cb"}
// Dependencies: {}
extern "C" fn remote_create_cb (out : * mut * mut raw :: git_remote , repo : * mut raw :: git_repository , name : * const c_char , url : * const c_char , payload : * mut c_void ,) -> c_int { unsafe { let repo = Repository :: from_raw (repo) ; let code = panic :: wrap (| | { let name = CStr :: from_ptr (name) . to_str () . unwrap () ; let url = CStr :: from_ptr (url) . to_str () . unwrap () ; let f = payload as * mut Box < RemoteCreate < '_ > > ; match (* f) (& repo , name , url) { Ok (remote) => { * out = crate :: remote :: remote_into_raw (remote) ; 0 } Err (e) => e . raw_code () , } }) ; mem :: forget (repo) ; code . unwrap_or (- 1) } }
};
}
