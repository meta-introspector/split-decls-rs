// Generated macro for PopoSigningKeyInputChoice (enum)
macro_rules! Depcrate_popPopoSigningKeyInputChoice {
() => {
// Module: crate::pop
// Provides: {"PopoSigningKeyInputChoice"}
// Dependencies: {}
# [doc = " The `POPOSigningKeyInput` type defined in [RFC 4211 Section 4.1] features an inline CHOICE"] # [doc = " definition that is implemented as the POPOSigningKeyInputChoice enum."] # [doc = ""] # [doc = " ```text"] # [doc = "       authInfo            CHOICE {"] # [doc = "        sender              [0] GeneralName,"] # [doc = "        publicKeyMAC        PKMACValue },"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.1"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum PopoSigningKeyInputChoice { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "true")] Sender (GeneralName) , PublicKeyMAC (PkMacValue) , }
};
}
