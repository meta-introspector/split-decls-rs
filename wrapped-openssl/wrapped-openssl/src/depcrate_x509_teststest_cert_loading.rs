// Generated macro for test_cert_loading (function)
macro_rules! Depcrate_x509_teststest_cert_loading {
() => {
// Module: crate::x509::tests
// Provides: {"test_cert_loading"}
// Dependencies: {}
# [test] fn test_cert_loading () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let fingerprint = cert . digest (MessageDigest :: sha1 ()) . unwrap () ; let hash_str = "59172d9313e84459bcff27f967e79e6e9217e584" ; let hash_vec = Vec :: from_hex (hash_str) . unwrap () ; assert_eq ! (hash_vec , &* fingerprint) ; }
};
}
