// Generated macro for impl_80 (impl)
macro_rules! Depcrate_projectiveimpl_80 {
() => {
// Module: crate::projective
// Provides: {"impl_80"}
// Dependencies: {}
impl < C > From < PublicKey < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (public_key : PublicKey < C >) -> ProjectivePoint < C > { AffinePoint :: from (public_key) . into () } }
};
}
