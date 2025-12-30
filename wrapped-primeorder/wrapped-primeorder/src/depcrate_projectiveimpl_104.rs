// Generated macro for impl_104 (impl)
macro_rules! Depcrate_projectiveimpl_104 {
() => {
// Module: crate::projective
// Provides: {"impl_104"}
// Dependencies: {}
impl < C > Add < & ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : & ProjectivePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add (& self , other) } }
};
}
