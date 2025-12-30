// Generated macro for verify_untrusted (function)
macro_rules! Depcrate_ssl_testverify_untrusted {
() => {
// Module: crate::ssl::test
// Provides: {"verify_untrusted"}
// Dependencies: {}
# [test] fn verify_untrusted () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_verify (SslVerifyMode :: PEER) ; client . connect_err () ; }
};
}
