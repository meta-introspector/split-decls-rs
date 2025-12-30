// Generated macro for impl_55 (impl)
macro_rules! Depcrate_affineimpl_55 {
() => {
// Module: crate::affine
// Provides: {"impl_55"}
// Dependencies: {}
impl < C , S > Mul < S > for AffinePoint < C > where C : PrimeCurveParams , S : Borrow < Scalar < C > > , ProjectivePoint < C > : Double , { type Output = ProjectivePoint < C > ; # [inline] fn mul (self , scalar : S) -> ProjectivePoint < C > { ProjectivePoint :: < C > :: from (self) * scalar } }
};
}
