// Generated macro for CertBag (struct)
macro_rules! Depcrate_pkcs12CertBag {
() => {
// Module: crate::pkcs12
// Provides: {"CertBag"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write)] pub struct CertBag < 'a > { pub _cert_id : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (_cert_id)] pub cert_value : asn1 :: Explicit < CertType < 'a > , 0 > , }
};
}
