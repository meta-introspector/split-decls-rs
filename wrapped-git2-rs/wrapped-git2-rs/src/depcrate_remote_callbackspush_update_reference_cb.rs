// Generated macro for push_update_reference_cb (function)
macro_rules! Depcrate_remote_callbackspush_update_reference_cb {
() => {
// Module: crate::remote_callbacks
// Provides: {"push_update_reference_cb"}
// Dependencies: {}
extern "C" fn push_update_reference_cb (refname : * const c_char , status : * const c_char , data : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let payload = & mut * (data as * mut RemoteCallbacks < '_ >) ; let callback = match payload . push_update_reference { Some (ref mut c) => c , None => return 0 , } ; let refname = str :: from_utf8 (CStr :: from_ptr (refname) . to_bytes ()) . unwrap () ; let status = if status . is_null () { None } else { Some (str :: from_utf8 (CStr :: from_ptr (status) . to_bytes ()) . unwrap ()) } ; match callback (refname , status) { Ok (()) => 0 , Err (e) => e . raw_set_git_error () , } }) . unwrap_or (- 1) }
};
}
