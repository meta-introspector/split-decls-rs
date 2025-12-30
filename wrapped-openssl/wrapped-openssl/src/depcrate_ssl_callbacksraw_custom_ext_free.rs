// Generated macro for raw_custom_ext_free (function)
macro_rules! Depcrate_ssl_callbacksraw_custom_ext_free {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_custom_ext_free"}
// Dependencies: {}
# [cfg (ossl111)] pub extern "C" fn raw_custom_ext_free < T > (ssl : * mut ffi :: SSL , _ : c_uint , _ : c_uint , _ : * const c_uchar , _ : * mut c_void ,) where T : 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let idx = Ssl :: cached_ex_index :: < CustomExtAddState < T > > () ; if let Some (state) = ssl . ex_data_mut (idx) { state . 0 = None ; } } }
};
}
