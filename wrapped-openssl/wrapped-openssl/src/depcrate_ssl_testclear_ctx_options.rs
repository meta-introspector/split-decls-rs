// Generated macro for clear_ctx_options (function)
macro_rules! Depcrate_ssl_testclear_ctx_options {
() => {
// Module: crate::ssl::test
// Provides: {"clear_ctx_options"}
// Dependencies: {}
# [test] # [cfg (not (any (boringssl , awslc)))] fn clear_ctx_options () { let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_options (SslOptions :: ALL) ; let opts = ctx . clear_options (SslOptions :: ALL) ; assert ! (! opts . contains (SslOptions :: ALL)) ; }
};
}
