// Generated macro for invoke_passwd_cb (function)
macro_rules! Depcrate_utilinvoke_passwd_cb {
() => {
// Module: crate::util
// Provides: {"invoke_passwd_cb"}
// Dependencies: {}
# [doc = " Password callback function, passed to private key loading functions."] # [doc = ""] # [doc = " `cb_state` is expected to be a pointer to a `CallbackState`."] pub unsafe extern "C" fn invoke_passwd_cb < F > (buf : * mut c_char , size : c_int , _rwflag : c_int , cb_state : * mut c_void ,) -> c_int where F : FnOnce (& mut [u8]) -> Result < usize , ErrorStack > , { let callback = & mut * (cb_state as * mut CallbackState < F >) ; let result = panic :: catch_unwind (AssertUnwindSafe (| | { let pass_slice = util :: from_raw_parts_mut (buf as * mut u8 , size as usize) ; callback . cb . take () . unwrap () (pass_slice) })) ; match result { Ok (Ok (len)) => len as c_int , Ok (Err (_)) => { 0 } Err (err) => { callback . panic = Some (err) ; 0 } } }
};
}
