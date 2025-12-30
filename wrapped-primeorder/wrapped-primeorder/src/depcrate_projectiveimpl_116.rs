// Generated macro for impl_116 (impl)
macro_rules! Depcrate_projectiveimpl_116 {
() => {
// Module: crate::projective
// Provides: {"impl_116"}
// Dependencies: {}
impl < C > Sub < & ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn sub (self , other : & ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: sub (& self , other) } }
};
}
