// Generated macro for impl_103 (impl)
macro_rules! Depcrate_signingimpl_103 {
() => {
// Module: crate::signing
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C > RandomizedSigner < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < der :: Signature < C > > { RandomizedSigner :: < Signature < C > > :: try_sign_with_rng (self , rng , msg) . map (Into :: into) } }
};
}
