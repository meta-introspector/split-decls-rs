// Generated macro for test_stack_from_pem (function)
macro_rules! Depcrate_x509_teststest_stack_from_pem {
() => {
// Module: crate::x509::tests
// Provides: {"test_stack_from_pem"}
// Dependencies: {}
# [test] fn test_stack_from_pem () { let certs = include_bytes ! ("../../test/certs.pem") ; let certs = X509 :: stack_from_pem (certs) . unwrap () ; assert_eq ! (certs . len () , 2) ; assert_eq ! (hex :: encode (certs [0] . digest (MessageDigest :: sha1 ()) . unwrap ()) , "59172d9313e84459bcff27f967e79e6e9217e584") ; assert_eq ! (hex :: encode (certs [1] . digest (MessageDigest :: sha1 ()) . unwrap ()) , "c0cbdf7cdd03c9773e5468e1f6d2da7d5cbb1875") ; }
};
}
