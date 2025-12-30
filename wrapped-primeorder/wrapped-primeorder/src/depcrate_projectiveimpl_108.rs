// Generated macro for impl_108 (impl)
macro_rules! Depcrate_projectiveimpl_108 {
() => {
// Module: crate::projective
// Provides: {"impl_108"}
// Dependencies: {}
impl < C > Add < & AffinePoint < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , { type Output = ProjectivePoint < C > ; fn add (self , other : & AffinePoint < C >) -> ProjectivePoint < C > { ProjectivePoint :: add_mixed (self , other) } }
};
}
