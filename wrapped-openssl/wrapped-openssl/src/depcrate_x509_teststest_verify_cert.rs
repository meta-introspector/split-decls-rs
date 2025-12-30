// Generated macro for test_verify_cert (function)
macro_rules! Depcrate_x509_teststest_verify_cert {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_cert"}
// Dependencies: {}
# [test] fn test_verify_cert () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; store_bldr . add_cert (ca) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert ! (context . init (& store , & cert , & chain , | c | c . verify_cert ()) . unwrap ()) ; assert ! (context . init (& store , & cert , & chain , | c | c . verify_cert ()) . unwrap ()) ; }
};
}
