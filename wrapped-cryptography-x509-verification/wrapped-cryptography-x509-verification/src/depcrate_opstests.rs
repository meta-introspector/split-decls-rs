// Generated macro for tests (module)
macro_rules! Depcrate_opstests {
() => {
// Module: crate::ops
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod tests { use cryptography_x509 :: certificate :: Certificate ; use super :: VerificationCertificate ; use crate :: certificate :: tests :: PublicKeyErrorOps ; pub (crate) fn v1_cert_pem () -> pem :: Pem { pem :: parse ("
-----BEGIN CERTIFICATE-----
MIIBWzCCAQYCARgwDQYJKoZIhvcNAQEEBQAwODELMAkGA1UEBhMCQVUxDDAKBgNV
BAgTA1FMRDEbMBkGA1UEAxMSU1NMZWF5L3JzYSB0ZXN0IENBMB4XDTk1MDYxOTIz
MzMxMloXDTk1MDcxNzIzMzMxMlowOjELMAkGA1UEBhMCQVUxDDAKBgNVBAgTA1FM
RDEdMBsGA1UEAxMUU1NMZWF5L3JzYSB0ZXN0IGNlcnQwXDANBgkqhkiG9w0BAQEF
AANLADBIAkEAqtt6qS5GTxVxGZYWa0/4u+IwHf7p2LNZbcPBp9/OfIcYAXBQn8hO
/Re1uwLKXdCjIoaGs4DLdG88rkzfyK5dPQIDAQABMAwGCCqGSIb3DQIFBQADQQAE
Wc7EcF8po2/ZO6kNCwK/ICH6DobgLekA5lSLr5EvuioZniZp5lFzAw4+YzPQ7XKJ
zl9HYIMxATFyqSiD9jsx
-----END CERTIFICATE-----" ,) . unwrap () } pub (crate) fn epoch () -> asn1 :: DateTime { asn1 :: DateTime :: new (1970 , 1 , 1 , 0 , 0 , 0) . unwrap () } pub (crate) fn cert (cert_pem : & pem :: Pem) -> Certificate < '_ > { asn1 :: parse_single (cert_pem . contents ()) . unwrap () } # [test] fn test_verification_certificate_debug () { let p = v1_cert_pem () ; let c = cert (& p) ; let vc = VerificationCertificate :: < PublicKeyErrorOps > :: new (& c , ()) ; assert_eq ! (format ! ("{:?}" , vc) , "VerificationCertificate") ; } }
};
}
