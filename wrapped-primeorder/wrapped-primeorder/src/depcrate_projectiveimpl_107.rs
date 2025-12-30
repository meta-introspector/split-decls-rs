// Generated macro for impl_107 (impl)
macro_rules! Depcrate_projectiveimpl_107 {
() => {
// Module: crate::projective
// Provides: {"impl_107"}
// Dependencies: {}
impl < C > Add < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add_mixed (& self , & other) } }
};
}
