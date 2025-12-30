// Generated macro for OptionalValidity (struct)
macro_rules! Depcrate_requestOptionalValidity {
() => {
// Module: crate::request
// Provides: {"OptionalValidity"}
// Dependencies: {}
# [doc = " The `OptionalValidity` type is defined in [RFC 4211 Section 5]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OptionalValidity ::= SEQUENCE {"] # [doc = "       notBefore  [0] Time OPTIONAL,"] # [doc = "       notAfter   [1] Time OPTIONAL } -- at least one MUST be present"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 5]: https://www.rfc-editor.org/rfc/rfc4211#section-5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OptionalValidity { # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , constructed = "false" , optional = "true")] pub not_before : Option < Time > , # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "false" , optional = "true")] pub not_after : Option < Time > , }
};
}
