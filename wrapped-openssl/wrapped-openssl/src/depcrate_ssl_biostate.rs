// Generated macro for state (function)
macro_rules! Depcrate_ssl_biostate {
() => {
// Module: crate::ssl::bio
// Provides: {"state"}
// Dependencies: {}
unsafe fn state < 'a , S : 'a > (bio : * mut BIO) -> & 'a mut StreamState < S > { & mut * (BIO_get_data (bio) as * mut _) }
};
}
