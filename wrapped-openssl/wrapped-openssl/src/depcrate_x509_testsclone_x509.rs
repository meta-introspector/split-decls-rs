// Generated macro for clone_x509 (function)
macro_rules! Depcrate_x509_testsclone_x509 {
() => {
// Module: crate::x509::tests
// Provides: {"clone_x509"}
// Dependencies: {}
# [test] # [allow (clippy :: redundant_clone)] fn clone_x509 () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; drop (cert . clone ()) ; }
};
}
