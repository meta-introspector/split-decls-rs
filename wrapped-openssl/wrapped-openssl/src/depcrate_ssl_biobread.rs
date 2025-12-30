// Generated macro for bread (function)
macro_rules! Depcrate_ssl_biobread {
() => {
// Module: crate::ssl::bio
// Provides: {"bread"}
// Dependencies: {}
unsafe extern "C" fn bread < S : Read > (bio : * mut BIO , buf : * mut c_char , len : c_int) -> c_int { BIO_clear_retry_flags (bio) ; let state = state :: < S > (bio) ; let buf = util :: from_raw_parts_mut (buf as * mut _ , len as usize) ; match catch_unwind (AssertUnwindSafe (| | state . stream . read (buf))) { Ok (Ok (len)) => len as c_int , Ok (Err (err)) => { if retriable_error (& err) { BIO_set_retry_read (bio) ; } state . error = Some (err) ; - 1 } Err (err) => { state . panic = Some (err) ; - 1 } } }
};
}
