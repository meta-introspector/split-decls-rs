// Generated macro for set_ctx_options (function)
macro_rules! Depcrate_ssl_testset_ctx_options {
() => {
// Module: crate::ssl::test
// Provides: {"set_ctx_options"}
// Dependencies: {}
# [test] fn set_ctx_options () { let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; let opts = ctx . set_options (SslOptions :: NO_TICKET) ; assert ! (opts . contains (SslOptions :: NO_TICKET)) ; }
};
}
