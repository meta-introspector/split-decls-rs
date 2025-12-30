// Generated macro for x509_ref_version (function)
macro_rules! Depcrate_x509_testsx509_ref_version {
() => {
// Module: crate::x509::tests
// Provides: {"x509_ref_version"}
// Dependencies: {}
# [cfg (ossl110)] # [test] fn x509_ref_version () { let mut builder = X509Builder :: new () . unwrap () ; let expected_version = 2 ; builder . set_version (expected_version) . expect ("Failed to set certificate version") ; let cert = builder . build () ; let actual_version = cert . version () ; assert_eq ! (expected_version , actual_version , "Obtained certificate version is incorrect" ,) ; }
};
}
