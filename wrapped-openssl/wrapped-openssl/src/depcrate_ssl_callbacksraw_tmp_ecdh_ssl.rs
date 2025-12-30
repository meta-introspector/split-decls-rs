// Generated macro for raw_tmp_ecdh_ssl (function)
macro_rules! Depcrate_ssl_callbacksraw_tmp_ecdh_ssl {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_tmp_ecdh_ssl"}
// Dependencies: {}
# [cfg (all (ossl102 , not (ossl110)))] pub unsafe extern "C" fn raw_tmp_ecdh_ssl < F > (ssl : * mut ffi :: SSL , is_export : c_int , keylength : c_int ,) -> * mut ffi :: EC_KEY where F : Fn (& mut SslRef , bool , u32) -> Result < EcKey < Params > , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ex_data (Ssl :: cached_ex_index :: < Arc < F > > ()) . expect ("BUG: ssl tmp ecdh callback missing") . clone () ; match callback (ssl , is_export != 0 , keylength as u32) { Ok (ec_key) => { let ptr = ec_key . as_ptr () ; mem :: forget (ec_key) ; ptr } Err (e) => { e . put () ; ptr :: null_mut () } } }
};
}
