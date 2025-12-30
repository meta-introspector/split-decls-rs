// Generated macro for impl_148 (impl)
macro_rules! Depcrate_verifyingimpl_148 {
() => {
// Module: crate::verifying
// Provides: {"impl_148"}
// Dependencies: {}
# [cfg (feature = "der")] impl < C > PrehashVerifier < der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn verify_prehash (& self , prehash : & [u8] , signature : & der :: Signature < C >) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; PrehashVerifier :: < Signature < C > > :: verify_prehash (self , prehash , & signature) } }
};
}
