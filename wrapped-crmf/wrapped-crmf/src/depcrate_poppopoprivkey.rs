// Generated macro for POPOPrivKey (enum)
macro_rules! Depcrate_popPOPOPrivKey {
() => {
// Module: crate::pop
// Provides: {"POPOPrivKey"}
// Dependencies: {}
# [doc = " The `POPOPrivKey` type is defined in [RFC 4211 Section 4.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   POPOPrivKey ::= CHOICE {"] # [doc = "       thisMessage       [0] BIT STRING,         -- Deprecated"] # [doc = "       subsequentMessage [1] SubsequentMessage,"] # [doc = "       dhMAC             [2] BIT STRING,         -- Deprecated"] # [doc = "       agreeMAC          [3] PKMACValue,"] # [doc = "       encryptedKey      [4] EnvelopedData }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.2]: https://www.rfc-editor.org/rfc/rfc4211#section-4.2"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] # [allow (missing_docs)] pub enum POPOPrivKey { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "false")] ThisMessage (BitString) , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true")] SubsequentMessage (SubsequentMessage) , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "false")] DhMac (BitString) , # [asn1 (context_specific = "3" , tag_mode = "EXPLICIT" , constructed = "true")] AgreeMac (PkMacValue) , # [asn1 (context_specific = "4" , tag_mode = "EXPLICIT" , constructed = "true")] EncryptedKey (EnvelopedData) , }
};
}
