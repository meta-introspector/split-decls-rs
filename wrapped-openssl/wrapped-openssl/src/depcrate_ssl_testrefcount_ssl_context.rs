// Generated macro for refcount_ssl_context (function)
macro_rules! Depcrate_ssl_testrefcount_ssl_context {
() => {
// Module: crate::ssl::test
// Provides: {"refcount_ssl_context"}
// Dependencies: {}
# [test] fn refcount_ssl_context () { let mut ssl = { let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ssl :: Ssl :: new (& ctx . build ()) . unwrap () } ; { let new_ctx_a = SslContext :: builder (SslMethod :: tls ()) . unwrap () . build () ; ssl . set_ssl_context (& new_ctx_a) . unwrap () ; } }
};
}
