// Generated macro for test_dist_point_null (function)
macro_rules! Depcrate_x509_teststest_dist_point_null {
() => {
// Module: crate::x509::tests
// Provides: {"test_dist_point_null"}
// Dependencies: {}
# [test] fn test_dist_point_null () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; assert ! (cert . crl_distribution_points () . is_none ()) ; }
};
}
