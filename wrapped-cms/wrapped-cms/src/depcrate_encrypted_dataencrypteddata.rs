// Generated macro for EncryptedData (struct)
macro_rules! Depcrate_encrypted_dataEncryptedData {
() => {
// Module: crate::encrypted_data
// Provides: {"EncryptedData"}
// Dependencies: {}
# [doc = " The `EncryptedData` type is defined in [RFC 5652 Section 8]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EncryptedData ::= SEQUENCE {"] # [doc = "       version CMSVersion,"] # [doc = "       encryptedContentInfo EncryptedContentInfo,"] # [doc = "       ...,"] # [doc = "       [[2: unprotectedAttrs [1] IMPLICIT Attributes"] # [doc = "           {{UnprotectedEncAttributes}} OPTIONAL ]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 8]: https://www.rfc-editor.org/rfc/rfc5652#section-8"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncryptedData { pub version : CmsVersion , pub enc_content_info : EncryptedContentInfo , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub unprotected_attrs : Option < Attributes > , }
};
}
