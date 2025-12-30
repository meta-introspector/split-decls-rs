// Generated macro for impl_122 (impl)
macro_rules! Depcrate_signingimpl_122 {
() => {
// Module: crate::signing
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > SignatureAlgorithmIdentifier for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , Signature < C > : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Params = AnyRef < 'static > ; const SIGNATURE_ALGORITHM_IDENTIFIER : AlgorithmIdentifier < Self :: Params > = Signature :: < C > :: ALGORITHM_IDENTIFIER ; }
};
}
