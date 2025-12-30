// Generated macro for impl_101 (impl)
macro_rules! Depcrate_signingimpl_101 {
() => {
// Module: crate::signing
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C > RandomizedPrehashSigner < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn sign_prehash_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < der :: Signature < C > > { RandomizedPrehashSigner :: < Signature < C > > :: sign_prehash_with_rng (self , rng , prehash) . map (Into :: into) } }
};
}
