// Generated macro for test_retrieve_pathlen (function)
macro_rules! Depcrate_x509_teststest_retrieve_pathlen {
() => {
// Module: crate::x509::tests
// Provides: {"test_retrieve_pathlen"}
// Dependencies: {}
# [test] # [cfg (any (ossl110 , boringssl , awslc))] fn test_retrieve_pathlen () { let cert = include_bytes ! ("../../test/root-ca.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; assert_eq ! (cert . pathlen () , None) ; let cert = include_bytes ! ("../../test/intermediate-ca.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; assert_eq ! (cert . pathlen () , Some (0)) ; let cert = include_bytes ! ("../../test/alt_name_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; assert_eq ! (cert . pathlen () , None) ; }
};
}
