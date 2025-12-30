// Generated macro for PBES2Params (struct)
macro_rules! Depcrate_commonPBES2Params {
() => {
// Module: crate::common
// Provides: {"PBES2Params"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone , Debug)] pub struct PBES2Params < 'a > { pub key_derivation_func : Box < AlgorithmIdentifier < 'a > > , pub encryption_scheme : Box < AlgorithmIdentifier < 'a > > , }
};
}
