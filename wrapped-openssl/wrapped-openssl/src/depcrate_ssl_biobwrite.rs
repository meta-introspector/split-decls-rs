// Generated macro for bwrite (function)
macro_rules! Depcrate_ssl_biobwrite {
() => {
// Module: crate::ssl::bio
// Provides: {"bwrite"}
// Dependencies: {}
unsafe extern "C" fn bwrite < S : Write > (bio : * mut BIO , buf : * const c_char , len : c_int) -> c_int { BIO_clear_retry_flags (bio) ; let state = state :: < S > (bio) ; let buf = util :: from_raw_parts (buf as * const _ , len as usize) ; match catch_unwind (AssertUnwindSafe (| | state . stream . write (buf))) { Ok (Ok (len)) => len as c_int , Ok (Err (err)) => { if retriable_error (& err) { BIO_set_retry_write (bio) ; } state . error = Some (err) ; - 1 } Err (err) => { state . panic = Some (err) ; - 1 } } }
};
}
