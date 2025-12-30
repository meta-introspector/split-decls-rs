// Generated macro for test_set_purpose (function)
macro_rules! Depcrate_x509_teststest_set_purpose {
() => {
// Module: crate::x509::tests
// Provides: {"test_set_purpose"}
// Dependencies: {}
# [test] # [cfg (any (ossl102 , boringssl , awslc))] fn test_set_purpose () { let cert = include_bytes ! ("../../test/leaf.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let intermediate_ca = include_bytes ! ("../../test/intermediate-ca.pem") ; let intermediate_ca = X509 :: from_pem (intermediate_ca) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let mut chain = Stack :: new () . unwrap () ; chain . push (intermediate_ca) . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; store_bldr . add_cert (ca) . unwrap () ; let mut verify_params = X509VerifyParam :: new () . unwrap () ; verify_params . set_purpose (X509PurposeId :: ANY) . unwrap () ; store_bldr . set_param (& verify_params) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert ! (context . init (& store , & cert , & chain , | c | c . verify_cert ()) . unwrap ()) ; }
};
}
