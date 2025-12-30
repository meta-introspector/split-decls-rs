// Generated macro for test_authority_key_id (function)
macro_rules! Depcrate_x509_teststest_authority_key_id {
() => {
// Module: crate::x509::tests
// Provides: {"test_authority_key_id"}
// Dependencies: {}
# [test] # [cfg (any (ossl110 , boringssl , awslc))] fn test_authority_key_id () { let cert = include_bytes ! ("../../test/certv3.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let authority_key_id = cert . authority_key_id () . unwrap () ; assert_eq ! (authority_key_id . as_slice () , & b"\x6C\xD3\xA5\x03\xAB\x0D\x5F\x2C\xC9\x8D\x8A\x9C\x88\xA7\x88\x77\xB8\x37\xFD\x9A" [..]) ; }
};
}
