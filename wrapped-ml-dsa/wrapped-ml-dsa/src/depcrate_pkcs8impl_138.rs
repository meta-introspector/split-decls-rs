// Generated macro for impl_138 (impl)
macro_rules! Depcrate_pkcs8impl_138 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_138"}
// Dependencies: {}
impl < P > SignatureAlgorithmIdentifier for VerifyingKey < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < P > :: ALGORITHM_IDENTIFIER ; }
};
}
