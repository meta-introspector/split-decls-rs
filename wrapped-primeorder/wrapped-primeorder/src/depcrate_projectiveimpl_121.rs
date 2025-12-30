// Generated macro for impl_121 (impl)
macro_rules! Depcrate_projectiveimpl_121 {
() => {
// Module: crate::projective
// Provides: {"impl_121"}
// Dependencies: {}
impl < C > Sub < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : & AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub_mixed (& self , other) } }
};
}
