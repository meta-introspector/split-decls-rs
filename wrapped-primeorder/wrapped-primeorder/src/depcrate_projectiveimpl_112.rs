// Generated macro for impl_112 (impl)
macro_rules! Depcrate_projectiveimpl_112 {
() => {
// Module: crate::projective
// Provides: {"impl_112"}
// Dependencies: {}
impl < C > Sum for ProjectivePoint < C > where C : PrimeCurveParams , { fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { iter . fold (ProjectivePoint :: IDENTITY , | a , b | a + b) } }
};
}
