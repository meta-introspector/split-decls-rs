// Generated macro for SignedData (struct)
macro_rules! Depcrate_signed_dataSignedData {
() => {
// Module: crate::signed_data
// Provides: {"SignedData"}
// Dependencies: {}
# [doc = " The `SignedData` type is defined in [RFC 5652 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   SignedData ::= SEQUENCE {"] # [doc = "       version CMSVersion,"] # [doc = "       digestAlgorithms SET OF DigestAlgorithmIdentifier,"] # [doc = "       encapContentInfo EncapsulatedContentInfo,"] # [doc = "       certificates [0] IMPLICIT CertificateSet OPTIONAL,"] # [doc = "       crls [1] IMPLICIT RevocationInfoChoices OPTIONAL,"] # [doc = "       signerInfos SignerInfos }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.1]: https://www.rfc-editor.org/rfc/rfc5652#section-5.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct SignedData { pub version : CmsVersion , pub digest_algorithms : DigestAlgorithmIdentifiers , pub encap_content_info : EncapsulatedContentInfo , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub certificates : Option < CertificateSet > , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , optional = "true")] pub crls : Option < RevocationInfoChoices > , pub signer_infos : SignerInfos , }
};
}
