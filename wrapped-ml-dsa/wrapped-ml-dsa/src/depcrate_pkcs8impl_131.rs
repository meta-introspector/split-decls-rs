// Generated macro for impl_131 (impl)
macro_rules! Depcrate_pkcs8impl_131 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_131"}
// Dependencies: {}
impl < P > AssociatedAlgorithmIdentifier for Signature < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifierRef < 'static > = P :: ALGORITHM_IDENTIFIER ; }
};
}
