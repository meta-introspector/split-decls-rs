// Generated macro for AuthEnvelopedData (struct)
macro_rules! Depcrate_authenveloped_dataAuthEnvelopedData {
() => {
// Module: crate::authenveloped_data
// Provides: {"AuthEnvelopedData"}
// Dependencies: {}
# [doc = " The `AuthEnvelopedData` type is defined in [RFC 5083 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " AuthEnvelopedData ::= SEQUENCE {"] # [doc = "     version CMSVersion,"] # [doc = "     originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,"] # [doc = "     recipientInfos RecipientInfos,"] # [doc = "     authEncryptedContentInfo EncryptedContentInfo,"] # [doc = "     authAttrs [1] IMPLICIT AuthAttributes OPTIONAL,"] # [doc = "     mac MessageAuthenticationCode,"] # [doc = "     unauthAttrs [2] IMPLICIT UnauthAttributes OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5083 Section 4]: https://www.rfc-editor.org/rfc/rfc5083#section-4"] # [derive (Clone , Debug , Sequence)] # [allow (missing_docs)] pub struct AuthEnvelopedData { pub version : CmsVersion , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub originator_info : Option < OriginatorInfo > , pub recip_infos : RecipientInfos , pub auth_encrypted_content_info : EncryptedContentInfo , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub auth_attrs : Option < AuthAttributes > , pub mac : MessageAuthenticationCode , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub unauth_attrs : Option < UnauthAttributes > , }
};
}
