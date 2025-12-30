// Generated macro for impl_100 (impl)
macro_rules! Depcrate_signingimpl_100 {
() => {
// Module: crate::signing
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C , D > RandomizedDigestSigner < D , der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_sign_digest_with_rng < R : TryCryptoRng + ? Sized , F : Fn (& mut D) -> Result < () > > (& self , rng : & mut R , f : F ,) -> Result < der :: Signature < C > > { RandomizedDigestSigner :: < D , Signature < C > > :: try_sign_digest_with_rng (self , rng , f) . map (Into :: into) } }
};
}
