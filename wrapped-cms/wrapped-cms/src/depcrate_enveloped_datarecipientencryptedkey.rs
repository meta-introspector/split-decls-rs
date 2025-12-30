// Generated macro for RecipientEncryptedKey (struct)
macro_rules! Depcrate_enveloped_dataRecipientEncryptedKey {
() => {
// Module: crate::enveloped_data
// Provides: {"RecipientEncryptedKey"}
// Dependencies: {}
# [doc = " The `RecipientEncryptedKey` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RecipientEncryptedKey ::= SEQUENCE {"] # [doc = "       rid KeyAgreeRecipientIdentifier,"] # [doc = "       encryptedKey EncryptedKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RecipientEncryptedKey { pub rid : KeyAgreeRecipientIdentifier , pub enc_key : EncryptedKey , }
};
}
