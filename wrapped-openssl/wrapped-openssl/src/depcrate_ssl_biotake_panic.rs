// Generated macro for take_panic (function)
macro_rules! Depcrate_ssl_biotake_panic {
() => {
// Module: crate::ssl::bio
// Provides: {"take_panic"}
// Dependencies: {}
pub unsafe fn take_panic < S > (bio : * mut BIO) -> Option < Box < dyn Any + Send > > { let state = state :: < S > (bio) ; state . panic . take () }
};
}
