// Generated macro for PkMacValue (struct)
macro_rules! Depcrate_popPkMacValue {
() => {
// Module: crate::pop
// Provides: {"PkMacValue"}
// Dependencies: {}
# [doc = " The `PKMACValue` type is defined in [RFC 4211 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PKMACValue ::= SEQUENCE {"] # [doc = "       algId  AlgorithmIdentifier{MAC-ALGORITHM,"] # [doc = "                  {Password-MACAlgorithms}},"] # [doc = "       value  BIT STRING }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PkMacValue { pub alg_id : AlgorithmIdentifierOwned , pub value : BitString , }
};
}
