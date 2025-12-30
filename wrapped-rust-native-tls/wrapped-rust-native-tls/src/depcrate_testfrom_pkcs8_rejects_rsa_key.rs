// Generated macro for from_pkcs8_rejects_rsa_key (function)
macro_rules! Depcrate_testfrom_pkcs8_rejects_rsa_key {
() => {
// Module: crate::test
// Provides: {"from_pkcs8_rejects_rsa_key"}
// Dependencies: {}
# [test] fn from_pkcs8_rejects_rsa_key () { let keys = test_cert_gen :: keys () ; let cert = keys . server . cert_and_key . cert . to_pem () . into_bytes () ; let rsa_key = keys . server . cert_and_key . key . to_pem_incorrect () ; assert ! (Identity :: from_pkcs8 (& cert , rsa_key . as_bytes ()) . is_err ()) ; let pkcs8_key = rsa_to_pkcs8 (& rsa_key) ; assert ! (Identity :: from_pkcs8 (& cert , pkcs8_key . as_bytes ()) . is_ok ()) ; }
};
}
