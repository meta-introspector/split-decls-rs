// Generated macro for CertType (enum)
macro_rules! Depcrate_pkcs12CertType {
() => {
// Module: crate::pkcs12
// Provides: {"CertType"}
// Dependencies: {}
# [derive (asn1 :: Asn1DefinedByWrite)] pub enum CertType < 'a > { # [defined_by (X509_CERTIFICATE_OID)] X509 (asn1 :: OctetStringEncoded < crate :: certificate :: Certificate < 'a > >) , }
};
}
