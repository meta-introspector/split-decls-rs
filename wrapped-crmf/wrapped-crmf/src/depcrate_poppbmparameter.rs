// Generated macro for PbmParameter (struct)
macro_rules! Depcrate_popPbmParameter {
() => {
// Module: crate::pop
// Provides: {"PbmParameter"}
// Dependencies: {}
# [doc = " The `PBMParameter` type is defined in [RFC 4211 Section 4.4]."] # [doc = ""] # [doc = " ```text"] # [doc = "   PBMParameter ::= SEQUENCE {"] # [doc = "      salt                OCTET STRING,"] # [doc = "      owf                 AlgorithmIdentifier{DIGEST-ALGORITHM,"] # [doc = "                              {DigestAlgorithms}},"] # [doc = "      iterationCount      INTEGER,"] # [doc = "      mac                 AlgorithmIdentifier{MAC-ALGORITHM,"] # [doc = "                              {MACAlgorithms}}"] # [doc = "   }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.4]: https://www.rfc-editor.org/rfc/rfc4211#section-4.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct PbmParameter { pub salt : OctetString , pub owf : AlgorithmIdentifierOwned , pub iteration_count : u64 , pub mac : AlgorithmIdentifierOwned , }
};
}
