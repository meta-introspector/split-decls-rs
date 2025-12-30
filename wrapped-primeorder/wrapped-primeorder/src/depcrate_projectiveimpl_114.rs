// Generated macro for impl_114 (impl)
macro_rules! Depcrate_projectiveimpl_114 {
() => {
// Module: crate::projective
// Provides: {"impl_114"}
// Dependencies: {}
impl < C > Sub < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub (& self , & other) } }
};
}
