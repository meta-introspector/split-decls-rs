// Generated macro for impl_133 (impl)
macro_rules! Depcrate_pkcs8impl_133 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_133"}
// Dependencies: {}
impl < P > SignatureAlgorithmIdentifier for KeyPair < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < P > :: ALGORITHM_IDENTIFIER ; }
};
}
