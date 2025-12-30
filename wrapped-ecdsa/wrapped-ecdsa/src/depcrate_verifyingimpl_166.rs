// Generated macro for impl_166 (impl)
macro_rules! Depcrate_verifyingimpl_166 {
() => {
// Module: crate::verifying
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Params = ObjectIdentifier ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifier < ObjectIdentifier > = PublicKey :: < C > :: ALGORITHM_IDENTIFIER ; }
};
}
