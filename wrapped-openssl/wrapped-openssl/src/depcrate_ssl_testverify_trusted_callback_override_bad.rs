// Generated macro for verify_trusted_callback_override_bad (function)
macro_rules! Depcrate_ssl_testverify_trusted_callback_override_bad {
() => {
// Module: crate::ssl::test
// Provides: {"verify_trusted_callback_override_bad"}
// Dependencies: {}
# [test] fn verify_trusted_callback_override_bad () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_ca_file ("test/root-ca.pem") . unwrap () ; client . ctx () . set_verify_callback (SslVerifyMode :: PEER , | _ , _ | false) ; client . connect_err () ; }
};
}
