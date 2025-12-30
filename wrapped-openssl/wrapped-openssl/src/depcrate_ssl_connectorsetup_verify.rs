// Generated macro for setup_verify (function)
macro_rules! Depcrate_ssl_connectorsetup_verify {
() => {
// Module: crate::ssl::connector
// Provides: {"setup_verify"}
// Dependencies: {}
fn setup_verify (ctx : & mut SslContextBuilder) { ctx . set_verify (SslVerifyMode :: PEER) ; }
};
}
