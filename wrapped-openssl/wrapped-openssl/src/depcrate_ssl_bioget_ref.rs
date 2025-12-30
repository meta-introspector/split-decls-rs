// Generated macro for get_ref (function)
macro_rules! Depcrate_ssl_bioget_ref {
() => {
// Module: crate::ssl::bio
// Provides: {"get_ref"}
// Dependencies: {}
pub unsafe fn get_ref < 'a , S : 'a > (bio : * mut BIO) -> & 'a S { let state = & * (BIO_get_data (bio) as * const StreamState < S >) ; & state . stream }
};
}
