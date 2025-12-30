// Generated macro for raw_custom_ext_parse (function)
macro_rules! Depcrate_ssl_callbacksraw_custom_ext_parse {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_custom_ext_parse"}
// Dependencies: {}
# [cfg (ossl111)] pub extern "C" fn raw_custom_ext_parse < F > (ssl : * mut ffi :: SSL , _ : c_uint , context : c_uint , input : * const c_uchar , inlen : size_t , x : * mut ffi :: X509 , chainidx : size_t , al : * mut c_int , _ : * mut c_void ,) -> c_int where F : Fn (& mut SslRef , ExtensionContext , & [u8] , Option < (usize , & X509Ref) >) -> Result < () , SslAlert > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: custom ext parse callback missing") as * const F ; let ectx = ExtensionContext :: from_bits_truncate (context) ; # [allow (clippy :: unnecessary_cast)] let slice = util :: from_raw_parts (input as * const u8 , inlen) ; let cert = if ectx . contains (ExtensionContext :: TLS1_3_CERTIFICATE) { Some ((chainidx , X509Ref :: from_ptr (x))) } else { None } ; match (* callback) (ssl , ectx , slice , cert) { Ok (()) => 1 , Err (alert) => { * al = alert . 0 ; 0 } } } }
};
}
