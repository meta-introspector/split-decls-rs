// Generated macro for impl_120 (impl)
macro_rules! Depcrate_projectiveimpl_120 {
() => {
// Module: crate::projective
// Provides: {"impl_120"}
// Dependencies: {}
impl < C > Sub < & AffinePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : & AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub_mixed (self , other) } }
};
}
