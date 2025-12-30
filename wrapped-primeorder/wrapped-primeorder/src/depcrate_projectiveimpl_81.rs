// Generated macro for impl_81 (impl)
macro_rules! Depcrate_projectiveimpl_81 {
() => {
// Module: crate::projective
// Provides: {"impl_81"}
// Dependencies: {}
impl < C > From < & PublicKey < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (public_key : & PublicKey < C >) -> ProjectivePoint < C > { AffinePoint :: < C > :: from (public_key) . into () } }
};
}
