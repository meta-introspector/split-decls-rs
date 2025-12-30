// Generated macro for impl_136 (impl)
macro_rules! Depcrate_pkcs8impl_136 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_136"}
// Dependencies: {}
impl < P > SignatureAlgorithmIdentifier for SigningKey < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < P > :: ALGORITHM_IDENTIFIER ; }
};
}
