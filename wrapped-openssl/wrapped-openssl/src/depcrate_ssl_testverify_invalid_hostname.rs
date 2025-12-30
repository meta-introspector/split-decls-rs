// Generated macro for verify_invalid_hostname (function)
macro_rules! Depcrate_ssl_testverify_invalid_hostname {
() => {
// Module: crate::ssl::test
// Provides: {"verify_invalid_hostname"}
// Dependencies: {}
# [test] # [cfg (ossl102)] fn verify_invalid_hostname () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_ca_file ("test/root-ca.pem") . unwrap () ; client . ctx () . set_verify (SslVerifyMode :: PEER) ; let mut client = client . build () . builder () ; client . ssl () . param_mut () . set_hostflags (X509CheckFlags :: NO_PARTIAL_WILDCARDS) ; client . ssl () . param_mut () . set_host ("bogus.com") . unwrap () ; client . connect_err () ; }
};
}
