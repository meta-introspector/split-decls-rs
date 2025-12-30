// Generated macro for impl_127 (impl)
macro_rules! Depcrate_projectiveimpl_127 {
() => {
// Module: crate::projective
// Provides: {"impl_127"}
// Dependencies: {}
impl < C , S > MulAssign < S > for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { fn mul_assign (& mut self , scalar : S) { * self = ProjectivePoint :: mul (self , scalar . borrow ()) ; } }
};
}
