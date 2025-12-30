// Generated macro for test_load_crl_file_fail (function)
macro_rules! Depcrate_x509_teststest_load_crl_file_fail {
() => {
// Module: crate::x509::tests
// Provides: {"test_load_crl_file_fail"}
// Dependencies: {}
# [test] # [cfg (not (any (boringssl , awslc)))] fn test_load_crl_file_fail () { let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; let lookup = store_bldr . add_lookup (X509Lookup :: file ()) . unwrap () ; let res = lookup . load_crl_file ("test/root-ca.pem" , SslFiletype :: PEM) ; assert ! (res . is_err ()) ; }
};
}
