// Generated macro for get_mut (function)
macro_rules! Depcrate_ssl_bioget_mut {
() => {
// Module: crate::ssl::bio
// Provides: {"get_mut"}
// Dependencies: {}
pub unsafe fn get_mut < 'a , S : 'a > (bio : * mut BIO) -> & 'a mut S { & mut state (bio) . stream }
};
}
