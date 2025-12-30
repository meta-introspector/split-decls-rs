// Generated macro for impl_14 (impl)
macro_rules! Depcrate_recoveryimpl_14 {
() => {
// Module: crate::recovery
// Provides: {"impl_14"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > PrehashSigner < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash (& self , prehash : & [u8]) -> Result < (Signature < C > , RecoveryId) > { self . sign_prehash_recoverable (prehash) } }
};
}
