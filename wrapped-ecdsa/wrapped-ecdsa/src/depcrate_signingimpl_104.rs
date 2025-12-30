// Generated macro for impl_104 (impl)
macro_rules! Depcrate_signingimpl_104 {
() => {
// Module: crate::signing
// Provides: {"impl_104"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C > RandomizedMultipartSigner < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]] ,) -> Result < der :: Signature < C > > { RandomizedMultipartSigner :: < Signature < C > > :: try_multipart_sign_with_rng (self , rng , msg) . map (Into :: into) } }
};
}
