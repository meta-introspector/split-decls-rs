// Generated macro for verify_trusted_get_error_err (function)
macro_rules! Depcrate_ssl_testverify_trusted_get_error_err {
() => {
// Module: crate::ssl::test
// Provides: {"verify_trusted_get_error_err"}
// Dependencies: {}
# [test] fn verify_trusted_get_error_err () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_verify_callback (SslVerifyMode :: PEER , | _ , x509 | { assert_ne ! (x509 . error () , X509VerifyResult :: OK) ; false }) ; client . connect_err () ; }
};
}
