// Generated macro for impl_56 (impl)
macro_rules! Depcrate_affineimpl_56 {
() => {
// Module: crate::affine
// Provides: {"impl_56"}
// Dependencies: {}
impl < C , S > Mul < S > for & AffinePoint < C > where C : PrimeCurveParams , S : Borrow < Scalar < C > > , ProjectivePoint < C > : Double , { type Output = ProjectivePoint < C > ; # [inline] fn mul (self , scalar : S) -> ProjectivePoint < C > { ProjectivePoint :: < C > :: from (self) * scalar } }
};
}
