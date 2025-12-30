// Generated macro for test_load_cert_file (function)
macro_rules! Depcrate_x509_teststest_load_cert_file {
() => {
// Module: crate::x509::tests
// Provides: {"test_load_cert_file"}
// Dependencies: {}
# [test] # [cfg (not (any (boringssl , awslc)))] fn test_load_cert_file () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; let lookup = store_bldr . add_lookup (X509Lookup :: file ()) . unwrap () ; lookup . load_cert_file ("test/root-ca.pem" , SslFiletype :: PEM) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert ! (context . init (& store , & cert , & chain , | c | c . verify_cert ()) . unwrap ()) ; }
};
}
