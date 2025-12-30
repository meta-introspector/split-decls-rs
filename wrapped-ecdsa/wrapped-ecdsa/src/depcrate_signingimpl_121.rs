// Generated macro for impl_121 (impl)
macro_rules! Depcrate_signingimpl_121 {
() => {
// Module: crate::signing
// Provides: {"impl_121"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = SecretKey :: < C > :: ALGORITHM_IDENTIFIER ; }
};
}
