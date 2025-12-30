// Generated macro for test_subject_key_id (function)
macro_rules! Depcrate_x509_teststest_subject_key_id {
() => {
// Module: crate::x509::tests
// Provides: {"test_subject_key_id"}
// Dependencies: {}
# [test] # [cfg (any (ossl110 , boringssl , awslc))] fn test_subject_key_id () { let cert = include_bytes ! ("../../test/certv3.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject_key_id = cert . subject_key_id () . unwrap () ; assert_eq ! (subject_key_id . as_slice () , & b"\xB6\x73\x2F\x61\xA5\x4B\xA1\xEF\x48\x2C\x15\xB1\x9F\xF3\xDC\x34\x2F\xBC\xAC\x30" [..]) ; }
};
}
