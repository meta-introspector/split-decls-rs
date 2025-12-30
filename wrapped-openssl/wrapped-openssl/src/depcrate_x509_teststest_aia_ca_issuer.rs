// Generated macro for test_aia_ca_issuer (function)
macro_rules! Depcrate_x509_teststest_aia_ca_issuer {
() => {
// Module: crate::x509::tests
// Provides: {"test_aia_ca_issuer"}
// Dependencies: {}
# [test] fn test_aia_ca_issuer () { let cert = include_bytes ! ("../../test/aia_test_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let authority_info = cert . authority_info () . unwrap () ; assert_eq ! (authority_info . len () , 1) ; assert_eq ! (authority_info [0] . method () . to_string () , "CA Issuers") ; assert_eq ! (authority_info [0] . location () . uri () , Some ("http://www.example.com/cert.pem")) ; let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; assert ! (cert . authority_info () . is_none ()) ; }
};
}
