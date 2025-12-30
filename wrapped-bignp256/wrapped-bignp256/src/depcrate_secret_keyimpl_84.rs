// Generated macro for impl_84 (impl)
macro_rules! Depcrate_secret_keyimpl_84 {
() => {
// Module: crate::secret_key
// Provides: {"impl_84"}
// Dependencies: {}
impl AssociatedAlgorithmIdentifier for SecretKey { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = AlgorithmIdentifier { oid : ALGORITHM_OID , parameters : Some (BignP256 :: OID) , } ; }
};
}
