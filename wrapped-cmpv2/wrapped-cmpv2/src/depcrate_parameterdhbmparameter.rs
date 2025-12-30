// Generated macro for DhbmParameter (struct)
macro_rules! Depcrate_parameterDhbmParameter {
() => {
// Module: crate::parameter
// Provides: {"DhbmParameter"}
// Dependencies: {}
# [doc = " The `PBMParameter` type is defined in [RFC 4210 Section 5.1.3.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " DHBMParameter ::= SEQUENCE {"] # [doc = "     owf                 AlgorithmIdentifier{DIGEST-ALGORITHM, {...}},"] # [doc = "     mac                 AlgorithmIdentifier{MAC-ALGORITHM, {...}}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1.3.2]: https://www.rfc-editor.org/rfc/rfc4210#section-5.1.3.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct DhbmParameter { pub owf : AlgorithmIdentifierOwned , pub mac : AlgorithmIdentifierOwned , }
};
}
