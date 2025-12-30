// Generated macro for take_error (function)
macro_rules! Depcrate_ssl_biotake_error {
() => {
// Module: crate::ssl::bio
// Provides: {"take_error"}
// Dependencies: {}
pub unsafe fn take_error < S > (bio : * mut BIO) -> Option < io :: Error > { let state = state :: < S > (bio) ; state . error . take () }
};
}
