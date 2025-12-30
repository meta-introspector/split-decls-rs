// Generated macro for impl_98 (impl)
macro_rules! Depcrate_signingimpl_98 {
() => {
// Module: crate::signing
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C > PrehashSigner < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn sign_prehash (& self , prehash : & [u8]) -> Result < der :: Signature < C > > { PrehashSigner :: < Signature < C > > :: sign_prehash (self , prehash) . map (Into :: into) } }
};
}
