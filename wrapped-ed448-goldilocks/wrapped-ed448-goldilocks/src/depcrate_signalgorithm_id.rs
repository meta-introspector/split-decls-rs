// Generated macro for ALGORITHM_ID (const)
macro_rules! Depcrate_signALGORITHM_ID {
() => {
// Module: crate::sign
// Provides: {"ALGORITHM_ID"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] # [doc = " The `AlgorithmIdentifier` for Ed448 as defined in [RFC8410 §2]"] pub const ALGORITHM_ID : pkcs8 :: AlgorithmIdentifierRef < 'static > = pkcs8 :: AlgorithmIdentifierRef { oid : ALGORITHM_OID , parameters : None , } ;
};
}
