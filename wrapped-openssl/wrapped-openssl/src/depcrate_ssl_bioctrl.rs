// Generated macro for ctrl (function)
macro_rules! Depcrate_ssl_bioctrl {
() => {
// Module: crate::ssl::bio
// Provides: {"ctrl"}
// Dependencies: {}
unsafe extern "C" fn ctrl < S : Write > (bio : * mut BIO , cmd : c_int , _num : c_long , _ptr : * mut c_void ,) -> c_long { let state = state :: < S > (bio) ; if cmd == BIO_CTRL_FLUSH { match catch_unwind (AssertUnwindSafe (| | state . stream . flush ())) { Ok (Ok (())) => 1 , Ok (Err (err)) => { state . error = Some (err) ; 0 } Err (err) => { state . panic = Some (err) ; 0 } } } else if cmd == BIO_CTRL_DGRAM_QUERY_MTU { state . dtls_mtu_size } else { 0 } }
};
}
