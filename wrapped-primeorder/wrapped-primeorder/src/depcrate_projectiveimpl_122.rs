// Generated macro for impl_122 (impl)
macro_rules! Depcrate_projectiveimpl_122 {
() => {
// Module: crate::projective
// Provides: {"impl_122"}
// Dependencies: {}
impl < C > SubAssign < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : AffinePoint < C >) { * self = ProjectivePoint :: sub_mixed (self , & rhs) ; } }
};
}
