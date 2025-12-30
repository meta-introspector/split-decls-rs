// Generated macro for DigestedData (struct)
macro_rules! Depcrate_digested_dataDigestedData {
() => {
// Module: crate::digested_data
// Provides: {"DigestedData"}
// Dependencies: {}
# [doc = " The `DigestedData` type is defined in [RFC 5652 Section 7]."] # [doc = ""] # [doc = " ```text"] # [doc = "   DigestedData ::= SEQUENCE {"] # [doc = "       version CMSVersion,"] # [doc = "       digestAlgorithm DigestAlgorithmIdentifier,"] # [doc = "       encapContentInfo EncapsulatedContentInfo,"] # [doc = "       digest Digest"] # [doc = "   }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 7]: https://www.rfc-editor.org/rfc/rfc5652#section-7"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct DigestedData { pub version : CmsVersion , pub digest_alg : AlgorithmIdentifierOwned , pub encap_content_info : EncapsulatedContentInfo , pub digest : Digest , }
};
}
