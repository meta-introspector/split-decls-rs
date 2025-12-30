// Generated macro for impl_123 (impl)
macro_rules! Depcrate_projectiveimpl_123 {
() => {
// Module: crate::projective
// Provides: {"impl_123"}
// Dependencies: {}
impl < C > SubAssign < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : & AffinePoint < C >) { * self = ProjectivePoint :: sub_mixed (self , rhs) ; } }
};
}
