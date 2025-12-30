// Generated macro for impl_110 (impl)
macro_rules! Depcrate_projectiveimpl_110 {
() => {
// Module: crate::projective
// Provides: {"impl_110"}
// Dependencies: {}
impl < C > AddAssign < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : AffinePoint < C >) { * self = ProjectivePoint :: add_mixed (self , & rhs) ; } }
};
}
