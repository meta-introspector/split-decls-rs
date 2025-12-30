// Generated macro for impl_100 (impl)
macro_rules! Depcrate_projectiveimpl_100 {
() => {
// Module: crate::projective
// Provides: {"impl_100"}
// Dependencies: {}
impl < C > TryFrom < ProjectivePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (point : ProjectivePoint < C >) -> Result < PublicKey < C > > { AffinePoint :: < C > :: from (point) . try_into () } }
};
}
