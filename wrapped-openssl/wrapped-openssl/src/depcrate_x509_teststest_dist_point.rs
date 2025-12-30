// Generated macro for test_dist_point (function)
macro_rules! Depcrate_x509_teststest_dist_point {
() => {
// Module: crate::x509::tests
// Provides: {"test_dist_point"}
// Dependencies: {}
# [test] fn test_dist_point () { let cert = include_bytes ! ("../../test/certv3.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let dps = cert . crl_distribution_points () . unwrap () ; let dp = dps . get (0) . unwrap () ; let dp_nm = dp . distpoint () . unwrap () ; let dp_gns = dp_nm . fullname () . unwrap () ; let dp_gn = dp_gns . get (0) . unwrap () ; assert_eq ! (dp_gn . uri () . unwrap () , "http://example.com/crl.pem") ; let dp = dps . get (1) . unwrap () ; let dp_nm = dp . distpoint () . unwrap () ; let dp_gns = dp_nm . fullname () . unwrap () ; let dp_gn = dp_gns . get (0) . unwrap () ; assert_eq ! (dp_gn . uri () . unwrap () , "http://example.com/crl2.pem") ; assert ! (dps . get (2) . is_none ()) }
};
}
