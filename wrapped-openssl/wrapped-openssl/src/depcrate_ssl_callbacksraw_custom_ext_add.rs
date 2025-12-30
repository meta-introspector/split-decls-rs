// Generated macro for raw_custom_ext_add (function)
macro_rules! Depcrate_ssl_callbacksraw_custom_ext_add {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_custom_ext_add"}
// Dependencies: {}
# [cfg (ossl111)] pub extern "C" fn raw_custom_ext_add < F , T > (ssl : * mut ffi :: SSL , _ : c_uint , context : c_uint , out : * mut * const c_uchar , outlen : * mut size_t , x : * mut ffi :: X509 , chainidx : size_t , al : * mut c_int , _ : * mut c_void ,) -> c_int where F : Fn (& mut SslRef , ExtensionContext , Option < (usize , & X509Ref) >) -> Result < Option < T > , SslAlert > + 'static + Sync + Send , T : AsRef < [u8] > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: custom ext add callback missing") as * const F ; let ectx = ExtensionContext :: from_bits_truncate (context) ; let cert = if ectx . contains (ExtensionContext :: TLS1_3_CERTIFICATE) { Some ((chainidx , X509Ref :: from_ptr (x))) } else { None } ; match (* callback) (ssl , ectx , cert) { Ok (None) => 0 , Ok (Some (buf)) => { * outlen = buf . as_ref () . len () ; * out = buf . as_ref () . as_ptr () ; let idx = Ssl :: cached_ex_index :: < CustomExtAddState < T > > () ; let mut buf = Some (buf) ; let new = match ssl . ex_data_mut (idx) { Some (state) => { state . 0 = buf . take () ; false } None => true , } ; if new { ssl . set_ex_data (idx , CustomExtAddState (buf)) ; } 1 } Err (alert) => { * al = alert . 0 ; - 1 } } } }
};
}
