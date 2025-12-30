// Generated macro for import_same_identity_multiple_times (function)
macro_rules! Depcrate_testimport_same_identity_multiple_times {
() => {
// Module: crate::test
// Provides: {"import_same_identity_multiple_times"}
// Dependencies: {}
# [test] fn import_same_identity_multiple_times () { let keys = test_cert_gen :: keys () ; let _ = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let _ = p ! (Identity :: from_pkcs12 (& keys . server . cert_and_key_pkcs12 . pkcs12 . 0 , & keys . server . cert_and_key_pkcs12 . password)) ; let cert = keys . server . cert_and_key . cert . to_pem () . into_bytes () ; let key = rsa_to_pkcs8 (& keys . server . cert_and_key . key . to_pem_incorrect ()) . into_bytes () ; let _ = p ! (Identity :: from_pkcs8 (& cert , & key)) ; let _ = p ! (Identity :: from_pkcs8 (& cert , & key)) ; }
};
}
