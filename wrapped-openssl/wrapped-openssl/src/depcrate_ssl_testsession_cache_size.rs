// Generated macro for session_cache_size (function)
macro_rules! Depcrate_ssl_testsession_cache_size {
() => {
// Module: crate::ssl::test
// Provides: {"session_cache_size"}
// Dependencies: {}
# [test] fn session_cache_size () { let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_session_cache_size (1234) ; let ctx = ctx . build () ; assert_eq ! (ctx . session_cache_size () , 1234) ; }
};
}
