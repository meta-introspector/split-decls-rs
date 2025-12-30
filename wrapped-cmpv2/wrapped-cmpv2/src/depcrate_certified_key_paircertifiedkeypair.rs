// Generated macro for CertifiedKeyPair (struct)
macro_rules! Depcrate_certified_key_pairCertifiedKeyPair {
() => {
// Module: crate::certified_key_pair
// Provides: {"CertifiedKeyPair"}
// Dependencies: {}
# [doc = " The `CertifiedKeyPair` type is defined in [RFC 4210 Section 5.3.4]"] # [doc = ""] # [doc = " ```text"] # [doc = "  CertifiedKeyPair ::= SEQUENCE {"] # [doc = "      certOrEncCert       CertOrEncCert,"] # [doc = "      privateKey      [0] EncryptedValue      OPTIONAL,"] # [doc = "      publicationInfo [1] PKIPublicationInfo  OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.3.4]: https://www.rfc-editor.org/rfc/rfc4210#section-5.3.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertifiedKeyPair { pub cert_or_enc_cert : CertOrEncCert , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub priv_key : Option < EncryptedValue > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true" , optional = "true")] pub publication_info : Option < PkiPublicationInfo > , }
};
}
