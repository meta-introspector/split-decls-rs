// Generated macro for EncryptedContentInfo (struct)
macro_rules! Depcrate_enveloped_dataEncryptedContentInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"EncryptedContentInfo"}
// Dependencies: {}
# [doc = " The `EncryptedContentInfo` type is defined in [RFC 5652 Section 6.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EncryptedContentInfo ::= SEQUENCE {"] # [doc = "       contentType ContentType,"] # [doc = "       contentEncryptionAlgorithm ContentEncryptionAlgorithmIdentifier,"] # [doc = "       encryptedContent [0] IMPLICIT EncryptedContent OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncryptedContentInfo { pub content_type : ObjectIdentifier , pub content_enc_alg : AlgorithmIdentifierOwned , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub encrypted_content : Option < OctetString > , }
};
}
