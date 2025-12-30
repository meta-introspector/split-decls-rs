// Generated macro for x509_ref_version_no_version_set (function)
macro_rules! Depcrate_x509_testsx509_ref_version_no_version_set {
() => {
// Module: crate::x509::tests
// Provides: {"x509_ref_version_no_version_set"}
// Dependencies: {}
# [cfg (ossl110)] # [test] fn x509_ref_version_no_version_set () { let cert = X509Builder :: new () . unwrap () . build () ; let actual_version = cert . version () ; assert_eq ! (0 , actual_version , "Default certificate version is incorrect" ,) ; }
};
}
