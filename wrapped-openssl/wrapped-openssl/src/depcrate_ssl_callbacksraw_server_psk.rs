// Generated macro for raw_server_psk (function)
macro_rules! Depcrate_ssl_callbacksraw_server_psk {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_server_psk"}
// Dependencies: {}
# [cfg (not (osslconf = "OPENSSL_NO_PSK"))] pub extern "C" fn raw_server_psk < F > (ssl : * mut ffi :: SSL , identity : * const c_char , psk : * mut c_uchar , max_psk_len : c_uint ,) -> c_uint where F : Fn (& mut SslRef , Option < & [u8] > , & mut [u8]) -> Result < usize , ErrorStack > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback_idx = SslContext :: cached_ex_index :: < F > () ; let callback = ssl . ssl_context () . ex_data (callback_idx) . expect ("BUG: psk callback missing") as * const F ; let identity = if identity . is_null () { None } else { Some (CStr :: from_ptr (identity) . to_bytes ()) } ; # [allow (clippy :: unnecessary_cast)] let psk_sl = util :: from_raw_parts_mut (psk as * mut u8 , max_psk_len as usize) ; match (* callback) (ssl , identity , psk_sl) { Ok (psk_len) => psk_len as u32 , Err (e) => { e . put () ; 0 } } } }
};
}
