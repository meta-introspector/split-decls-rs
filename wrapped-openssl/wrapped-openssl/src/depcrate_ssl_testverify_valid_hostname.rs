// Generated macro for verify_valid_hostname (function)
macro_rules! Depcrate_ssl_testverify_valid_hostname {
() => {
// Module: crate::ssl::test
// Provides: {"verify_valid_hostname"}
// Dependencies: {}
# [test] # [cfg (ossl102)] fn verify_valid_hostname () { let server = Server :: builder () . build () ; let mut client = server . client () ; client . ctx () . set_ca_file ("test/root-ca.pem") . unwrap () ; client . ctx () . set_verify (SslVerifyMode :: PEER) ; let mut client = client . build () . builder () ; client . ssl () . param_mut () . set_hostflags (X509CheckFlags :: NO_PARTIAL_WILDCARDS) ; client . ssl () . param_mut () . set_host ("foobar.com") . unwrap () ; client . connect () ; }
};
}
