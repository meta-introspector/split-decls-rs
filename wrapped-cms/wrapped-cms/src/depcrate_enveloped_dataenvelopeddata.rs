// Generated macro for EnvelopedData (struct)
macro_rules! Depcrate_enveloped_dataEnvelopedData {
() => {
// Module: crate::enveloped_data
// Provides: {"EnvelopedData"}
// Dependencies: {}
# [doc = " The `EnvelopedData` type is defined in [RFC 5652 Section 6.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EnvelopedData ::= SEQUENCE {"] # [doc = "       version CMSVersion,"] # [doc = "       originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,"] # [doc = "       recipientInfos RecipientInfos,"] # [doc = "       encryptedContentInfo EncryptedContentInfo,"] # [doc = "       ...,"] # [doc = "       [[2: unprotectedAttrs [1] IMPLICIT Attributes"] # [doc = "           {{ UnprotectedEnvAttributes }} OPTIONAL ]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EnvelopedData { pub version : CmsVersion , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub originator_info : Option < OriginatorInfo > , pub recip_infos : RecipientInfos , pub encrypted_content : EncryptedContentInfo , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub unprotected_attrs : Option < Attributes > , }
};
}
