// Generated macro for impl_126 (impl)
macro_rules! Depcrate_projectiveimpl_126 {
() => {
// Module: crate::projective
// Provides: {"impl_126"}
// Dependencies: {}
impl < C > Mul < & Scalar < C > > for & ProjectivePoint < C > where C : PrimeCurveParams , ProjectivePoint < C > : Double , { type Output = ProjectivePoint < C > ; fn mul (self , scalar : & Scalar < C >) -> ProjectivePoint < C > { ProjectivePoint :: mul (self , scalar) } }
};
}
