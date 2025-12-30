// Generated macro for EncryptedKey (enum)
macro_rules! Depcrate_controlsEncryptedKey {
() => {
// Module: crate::controls
// Provides: {"EncryptedKey"}
// Dependencies: {}
# [doc = " The `EncryptedKey` type is defined in [RFC 4211 Section 6.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EncryptedKey ::= CHOICE {"] # [doc = "       encryptedValue        EncryptedValue,   -- Deprecated"] # [doc = "       envelopedData     [0] EnvelopedData }"] # [doc = "       -- The encrypted private key MUST be placed in the envelopedData"] # [doc = "       -- encryptedContentInfo encryptedContent OCTET STRING."] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6.4]: https://www.rfc-editor.org/rfc/rfc4211#section-6.4"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] # [allow (missing_docs)] pub enum EncryptedKey { EncryptedValue (Box < EncryptedValue >) , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true")] EnvelopedData (Box < EnvelopedData >) , }
};
}
