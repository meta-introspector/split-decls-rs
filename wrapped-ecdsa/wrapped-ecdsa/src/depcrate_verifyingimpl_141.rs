// Generated macro for impl_141 (impl)
macro_rules! Depcrate_verifyingimpl_141 {
() => {
// Module: crate::verifying
// Provides: {"impl_141"}
// Dependencies: {}
impl < C , D > DigestVerifier < D , Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , D : EagerHash + Update , SignatureSize < C > : ArraySize , { fn verify_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F , signature : & Signature < C > ,) -> Result < () > { let mut digest = D :: new () ; f (& mut digest) ? ; self . verify_prehash (& digest . finalize () , signature) } }
};
}
