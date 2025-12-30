// Generated macro for PopoSigningKey (struct)
macro_rules! Depcrate_popPopoSigningKey {
() => {
// Module: crate::pop
// Provides: {"PopoSigningKey"}
// Dependencies: {}
# [doc = " The `POPOSigningKey` type is defined in [RFC 4211 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   POPOSigningKey ::= SEQUENCE {"] # [doc = "       poposkInput           [0] POPOSigningKeyInput OPTIONAL,"] # [doc = "       algorithmIdentifier   AlgorithmIdentifier{SIGNATURE-ALGORITHM,"] # [doc = "                                 {SignatureAlgorithms}},"] # [doc = "       signature             BIT STRING }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PopoSigningKey { # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "true" , optional = "true")] pub poposk_input : Option < PopoSigningKeyInput > , pub alg_id : AlgorithmIdentifierOwned , pub signature : BitString , }
};
}
