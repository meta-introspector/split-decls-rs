// Generated macro for impl_79 (impl)
macro_rules! Depcrate_projectiveimpl_79 {
() => {
// Module: crate::projective
// Provides: {"impl_79"}
// Dependencies: {}
impl < C > From < NonIdentity < ProjectivePoint < C > > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : NonIdentity < ProjectivePoint < C > >) -> Self { p . to_point () } }
};
}
