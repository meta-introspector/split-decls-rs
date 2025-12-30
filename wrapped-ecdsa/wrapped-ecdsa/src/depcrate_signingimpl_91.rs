// Generated macro for impl_91 (impl)
macro_rules! Depcrate_signingimpl_91 {
() => {
// Module: crate::signing
// Provides: {"impl_91"}
// Dependencies: {}
impl < C , D > RandomizedDigestSigner < D , Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < Signature < C > > { let mut digest = D :: new () ; f (& mut digest) ? ; self . sign_prehash_with_rng (rng , & digest . finalize ()) } }
};
}
