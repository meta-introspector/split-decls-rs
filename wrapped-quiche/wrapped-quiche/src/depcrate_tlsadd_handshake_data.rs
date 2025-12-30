// Generated macro for add_handshake_data (function)
macro_rules! Depcrate_tlsadd_handshake_data {
() => {
// Module: crate::tls
// Provides: {"add_handshake_data"}
// Dependencies: {}
extern "C" fn add_handshake_data (ssl : * mut SSL , level : crypto :: Level , data : * const u8 , len : usize ,) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return 0 , } ; trace ! ("{} write message lvl={:?} len={}" , ex_data . trace_id , level , len) ; let buf = unsafe { slice :: from_raw_parts (data , len) } ; let space = match level { crypto :: Level :: Initial => & mut ex_data . crypto_ctx [packet :: Epoch :: Initial] , crypto :: Level :: ZeroRTT => unreachable ! () , crypto :: Level :: Handshake => & mut ex_data . crypto_ctx [packet :: Epoch :: Handshake] , crypto :: Level :: OneRTT => & mut ex_data . crypto_ctx [packet :: Epoch :: Application] , } ; if space . crypto_stream . send . write (buf , false) . is_err () { return 0 ; } 1 }
};
}
