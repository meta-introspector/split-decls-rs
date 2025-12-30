// Generated macro for verify_trusted (function)
macro_rules! Depcrate_ssl_testverify_trusted {
() => {
// Module: crate::ssl::test
// Provides: {"verify_trusted"}
// Dependencies: {}
# [test] fn verify_trusted () { let server = Server :: builder () . build () ; let mut client = server . client () ; client . ctx () . set_ca_file ("test/root-ca.pem") . unwrap () ; client . connect () ; }
};
}
