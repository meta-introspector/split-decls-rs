// Generated macro for impl_125 (impl)
macro_rules! Depcrate_projectiveimpl_125 {
() => {
// Module: crate::projective
// Provides: {"impl_125"}
// Dependencies: {}
impl < C , S > Mul < S > for & ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { type Output = ProjectivePoint < C > ; fn mul (self , scalar : S) -> ProjectivePoint < C > { ProjectivePoint :: mul (self , scalar . borrow ()) } }
};
}
