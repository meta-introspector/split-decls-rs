// Generated macro for CertificateChoices (enum)
macro_rules! Depcrate_certCertificateChoices {
() => {
// Module: crate::cert
// Provides: {"CertificateChoices"}
// Dependencies: {}
# [doc = " The `CertificateChoices` type is defined in [RFC 5652 Section 10.2.2]. Attribute certificate"] # [doc = " support is not presently implemented."] # [doc = ""] # [doc = " ```text"] # [doc = "   CertificateChoices ::= CHOICE {"] # [doc = "       certificate Certificate,"] # [doc = "       extendedCertificate [0] IMPLICIT ExtendedCertificate,"] # [doc = "            -- Obsolete"] # [doc = "       ...,"] # [doc = "       -- [[3: v1AttrCert [1] IMPLICIT AttributeCertificateV1]],"] # [doc = "            -- Obsolete"] # [doc = "       -- [[4: v2AttrCert [2] IMPLICIT AttributeCertificateV2]],"] # [doc = "       [[5: other      [3] IMPLICIT OtherCertificateFormat]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-10.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] # [allow (clippy :: large_enum_variant)] pub enum CertificateChoices { Certificate (Certificate) , # [asn1 (context_specific = "3" , tag_mode = "EXPLICIT" , constructed = "true")] Other (OtherCertificateFormat) , }
};
}
