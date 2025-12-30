// Generated macro for impl_291 (impl)
macro_rules! Depcrate_public_keyimpl_291 {
() => {
// Module: crate::public_key
// Provides: {"impl_291"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for PublicKey < C > where C : AssociatedOid + CurveArithmetic , { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = AlgorithmIdentifier { oid : ALGORITHM_OID , parameters : Some (C :: OID) , } ; }
};
}
