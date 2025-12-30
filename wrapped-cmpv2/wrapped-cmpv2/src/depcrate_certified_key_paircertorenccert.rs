// Generated macro for CertOrEncCert (enum)
macro_rules! Depcrate_certified_key_pairCertOrEncCert {
() => {
// Module: crate::certified_key_pair
// Provides: {"CertOrEncCert"}
// Dependencies: {}
# [doc = " The `CertOrEncCert` type is defined in [RFC 4210 Section 5.3.4]"] # [doc = ""] # [doc = " ```text"] # [doc = "  CertOrEncCert ::= CHOICE {"] # [doc = "      certificate     [0] CMPCertificate,"] # [doc = "      encryptedCert   [1] EncryptedValue }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.4]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.4"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum CertOrEncCert { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true")] Certificate (Box < CmpCertificate >) , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true")] EncryptedCert (Box < EncryptedValue >) , }
};
}
