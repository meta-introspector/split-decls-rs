// Generated macro for impl_15 (impl)
macro_rules! Depcrate_recoveryimpl_15 {
() => {
// Module: crate::recovery
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > Signer < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < (Signature < C > , RecoveryId) > { self . try_multipart_sign (& [msg]) } }
};
}
