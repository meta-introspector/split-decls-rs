// Generated macro for KeyAgreeRecipientIdentifier (enum)
macro_rules! Depcrate_enveloped_dataKeyAgreeRecipientIdentifier {
() => {
// Module: crate::enveloped_data
// Provides: {"KeyAgreeRecipientIdentifier"}
// Dependencies: {}
# [doc = " The `KeyAgreeRecipientIdentifier` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   KeyAgreeRecipientIdentifier ::= CHOICE {"] # [doc = "       issuerAndSerialNumber IssuerAndSerialNumber,"] # [doc = "       rKeyId [0] IMPLICIT RecipientKeyIdentifier }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum KeyAgreeRecipientIdentifier { IssuerAndSerialNumber (IssuerAndSerialNumber) , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true")] RKeyId (RecipientKeyIdentifier) , }
};
}
