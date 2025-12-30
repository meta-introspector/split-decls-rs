// Generated macro for impl_147 (impl)
macro_rules! Depcrate_verifyingimpl_147 {
() => {
// Module: crate::verifying
// Provides: {"impl_147"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C , D > DigestVerifier < D , der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , D : EagerHash + Update , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn verify_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F , signature : & der :: Signature < C > ,) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; DigestVerifier :: < D , Signature < C > > :: verify_digest (self , f , & signature) } }
};
}
