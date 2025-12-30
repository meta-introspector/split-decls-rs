// Generated macro for PasswordRecipientInfo (struct)
macro_rules! Depcrate_enveloped_dataPasswordRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"PasswordRecipientInfo"}
// Dependencies: {}
# [doc = " The `PasswordRecipientInfo` type is defined in [RFC 5652 Section 6.2.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PasswordRecipientInfo ::= SEQUENCE {"] # [doc = "       version CMSVersion,   -- always set to 0"] # [doc = "       keyDerivationAlgorithm [0] KeyDerivationAlgorithmIdentifier"] # [doc = "                               OPTIONAL,"] # [doc = "       keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,"] # [doc = "       encryptedKey EncryptedKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.4]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PasswordRecipientInfo { pub version : CmsVersion , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub key_derivation_alg : Option < AlgorithmIdentifierOwned > , pub key_enc_alg : AlgorithmIdentifierOwned , pub enc_key : EncryptedKey , }
};
}
