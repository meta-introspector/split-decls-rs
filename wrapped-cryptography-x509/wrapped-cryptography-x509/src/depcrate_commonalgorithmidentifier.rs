// Generated macro for AlgorithmIdentifier (struct)
macro_rules! Depcrate_commonAlgorithmIdentifier {
() => {
// Module: crate::common
// Provides: {"AlgorithmIdentifier"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Hash , Clone , Eq , Debug)] pub struct AlgorithmIdentifier < 'a > { pub oid : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (oid)] pub params : AlgorithmParameters < 'a > , }
};
}
