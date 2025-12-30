// Generated macro for test_load_crl (function)
macro_rules! Depcrate_x509_teststest_load_crl {
() => {
// Module: crate::x509::tests
// Provides: {"test_load_crl"}
// Dependencies: {}
# [test] fn test_load_crl () { let ca = include_bytes ! ("../../test/crl-ca.crt") ; let ca = X509 :: from_pem (ca) . unwrap () ; let crl = include_bytes ! ("../../test/test.crl") ; let crl = X509Crl :: from_der (crl) . unwrap () ; assert ! (crl . verify (& ca . public_key () . unwrap ()) . unwrap ()) ; let cert = include_bytes ! ("../../test/subca.crt") ; let cert = X509 :: from_pem (cert) . unwrap () ; let revoked = match crl . get_by_cert (& cert) { CrlStatus :: Revoked (revoked) => revoked , _ => panic ! ("cert should be revoked") , } ; assert_eq ! (revoked . serial_number () . to_bn () . unwrap () , cert . serial_number () . to_bn () . unwrap () , "revoked and cert serial numbers should match") ; }
};
}
