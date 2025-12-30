// Generated macro for impl_286 (impl)
macro_rules! Depcrate_keylogimpl_286 {
() => {
// Module: crate::keylog
// Provides: {"impl_286"}
// Dependencies: {}
impl rustls :: KeyLog for CallbackKeyLog { fn log (& self , label : & str , client_random : & [u8] , secret : & [u8]) { unsafe { (self . log_cb) (rustls_str :: try_from (label) . unwrap () , client_random . as_ptr () , client_random . len () , secret . as_ptr () , secret . len () ,) ; } } fn will_log (& self , label : & str) -> bool { match self . will_log_cb { Some (cb) => { let label = rustls_str :: try_from (label) . unwrap () ; ! matches ! (unsafe { (cb) (label) } , 0) } None => true , } } }
};
}
