// Generated macro for impl_111 (impl)
macro_rules! Depcrate_projectiveimpl_111 {
() => {
// Module: crate::projective
// Provides: {"impl_111"}
// Dependencies: {}
impl < C > AddAssign < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : & AffinePoint < C >) { * self = ProjectivePoint :: add_mixed (self , rhs) ; } }
};
}
