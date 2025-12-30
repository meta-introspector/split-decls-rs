// Generated macro for impl_12 (impl)
macro_rules! Depcrate_recoveryimpl_12 {
() => {
// Module: crate::recovery
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > RandomizedPrehashSigner < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn sign_prehash_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < (Signature < C > , RecoveryId) > { self . sign_prehash_recoverable_with_rng (rng , prehash) } }
};
}
