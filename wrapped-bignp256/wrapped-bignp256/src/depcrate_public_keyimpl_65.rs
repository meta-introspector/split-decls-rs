// Generated macro for impl_65 (impl)
macro_rules! Depcrate_public_keyimpl_65 {
() => {
// Module: crate::public_key
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl AssociatedAlgorithmIdentifier for PublicKey { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = AlgorithmIdentifier { oid : ALGORITHM_OID , parameters : Some (BignP256 :: OID) , } ; }
};
}
