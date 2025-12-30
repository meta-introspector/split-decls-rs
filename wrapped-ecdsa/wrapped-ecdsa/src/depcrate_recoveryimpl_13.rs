// Generated macro for impl_13 (impl)
macro_rules! Depcrate_recoveryimpl_13 {
() => {
// Module: crate::recovery
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C , D > RandomizedDigestSigner < D , (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + digest :: Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < (Signature < C > , RecoveryId) > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_prehash_with_rng (rng , & digest . finalize ()) } }
};
}
