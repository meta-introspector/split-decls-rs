// Generated macro for KeyTransRecipientInfo (struct)
macro_rules! Depcrate_enveloped_dataKeyTransRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"KeyTransRecipientInfo"}
// Dependencies: {}
# [doc = " The `KeyTransRecipientInfo` type is defined in [RFC 5652 Section 6.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   KeyTransRecipientInfo ::= SEQUENCE {"] # [doc = "       version CMSVersion,  -- always set to 0 or 2"] # [doc = "       rid RecipientIdentifier,"] # [doc = "       keyEncryptionAlgorithm AlgorithmIdentifier"] # [doc = "           {KEY-TRANSPORT, {KeyTransportAlgorithmSet}},"] # [doc = "       encryptedKey EncryptedKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KeyTransRecipientInfo { pub version : CmsVersion , pub rid : RecipientIdentifier , pub key_enc_alg : AlgorithmIdentifierOwned , pub enc_key : EncryptedKey , }
};
}
