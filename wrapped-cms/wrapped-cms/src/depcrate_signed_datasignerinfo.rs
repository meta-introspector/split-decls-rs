// Generated macro for SignerInfo (struct)
macro_rules! Depcrate_signed_dataSignerInfo {
() => {
// Module: crate::signed_data
// Provides: {"SignerInfo"}
// Dependencies: {}
# [doc = " The `SignerInfo` type is defined in [RFC 5652 Section 5.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   SignerInfo ::= SEQUENCE {"] # [doc = "       version CMSVersion,"] # [doc = "       sid SignerIdentifier,"] # [doc = "       digestAlgorithm DigestAlgorithmIdentifier,"] # [doc = "       signedAttrs [0] IMPLICIT SignedAttributes OPTIONAL,"] # [doc = "       signatureAlgorithm SignatureAlgorithmIdentifier,"] # [doc = "       signature SignatureValue,"] # [doc = "       unsignedAttrs [1] IMPLICIT Attributes"] # [doc = "           {{UnsignedAttributes}} OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.3]: https://www.rfc-editor.org/rfc/rfc5652#section-5.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct SignerInfo { pub version : CmsVersion , pub sid : SignerIdentifier , pub digest_alg : AlgorithmIdentifierOwned , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub signed_attrs : Option < SignedAttributes > , pub signature_algorithm : AlgorithmIdentifierOwned , pub signature : SignatureValue , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub unsigned_attrs : Option < UnsignedAttributes > , }
};
}
