// Generated macro for PbmParameter (struct)
macro_rules! Depcrate_parameterPbmParameter {
() => {
// Module: crate::parameter
// Provides: {"PbmParameter"}
// Dependencies: {}
# [doc = " The `PBMParameter` type is defined in [RFC 4210 Section 5.1.3.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " PBMParameter ::= SEQUENCE {"] # [doc = "     salt                OCTET STRING,"] # [doc = "     owf                 AlgorithmIdentifier{DIGEST-ALGORITHM, {...}},"] # [doc = "     iterationCount      INTEGER,"] # [doc = "     mac                 AlgorithmIdentifier{MAC-ALGORITHM, {...}}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1.3.1]: https://www.rfc-editor.org/rfc/rfc4210#section-5.1.3.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PbmParameter { pub salt : OctetString , pub owf : AlgorithmIdentifierOwned , pub iteration_count : u64 , pub mac : AlgorithmIdentifierOwned , }
};
}
