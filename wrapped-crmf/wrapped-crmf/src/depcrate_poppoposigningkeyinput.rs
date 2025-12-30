// Generated macro for PopoSigningKeyInput (struct)
macro_rules! Depcrate_popPopoSigningKeyInput {
() => {
// Module: crate::pop
// Provides: {"PopoSigningKeyInput"}
// Dependencies: {}
# [doc = " The `POPOSigningKeyInput` type is defined in [RFC 4211 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   POPOSigningKeyInput ::= SEQUENCE {"] # [doc = "       authInfo            CHOICE {"] # [doc = "        sender              [0] GeneralName,"] # [doc = "        publicKeyMAC        PKMACValue },"] # [doc = "       publicKey           SubjectPublicKeyInfo }  -- from CertTemplate"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PopoSigningKeyInput { pub auth_info : PopoSigningKeyInputChoice , pub public_key : SubjectPublicKeyInfoOwned , }
};
}
