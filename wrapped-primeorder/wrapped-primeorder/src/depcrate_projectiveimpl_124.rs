// Generated macro for impl_124 (impl)
macro_rules! Depcrate_projectiveimpl_124 {
() => {
// Module: crate::projective
// Provides: {"impl_124"}
// Dependencies: {}
impl < C , S > Mul < S > for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { type Output = Self ; fn mul (self , scalar : S) -> Self { ProjectivePoint :: mul (& self , scalar . borrow ()) } }
};
}
