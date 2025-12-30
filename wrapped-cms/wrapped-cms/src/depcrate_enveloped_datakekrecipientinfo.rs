// Generated macro for KekRecipientInfo (struct)
macro_rules! Depcrate_enveloped_dataKekRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"KekRecipientInfo"}
// Dependencies: {}
# [doc = " The `KEKRecipientInfo` type is defined in [RFC 5652 Section 6.2.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   KEKRecipientInfo ::= SEQUENCE {"] # [doc = "       version CMSVersion,  -- always set to 4"] # [doc = "       kekid KEKIdentifier,"] # [doc = "       keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,"] # [doc = "       encryptedKey EncryptedKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.3]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KekRecipientInfo { pub version : CmsVersion , pub kek_id : KekIdentifier , pub key_enc_alg : AlgorithmIdentifierOwned , pub encrypted_key : EncryptedKey , }
};
}
