// Generated macro for RevocationInfoChoice (enum)
macro_rules! Depcrate_revocationRevocationInfoChoice {
() => {
// Module: crate::revocation
// Provides: {"RevocationInfoChoice"}
// Dependencies: {}
# [doc = " The `RevocationInfoChoice` type is defined in [RFC 5652 Section 10.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RevocationInfoChoice ::= CHOICE {"] # [doc = "       crl CertificateList,"] # [doc = "       ...,"] # [doc = "       [[5: other [1] IMPLICIT OtherRevocationInfoFormat ]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.1]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] # [allow (clippy :: large_enum_variant)] pub enum RevocationInfoChoice { Crl (CertificateList) , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true")] Other (OtherRevocationInfoFormat) , }
};
}
