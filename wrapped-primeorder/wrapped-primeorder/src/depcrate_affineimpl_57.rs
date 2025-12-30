// Generated macro for impl_57 (impl)
macro_rules! Depcrate_affineimpl_57 {
() => {
// Module: crate::affine
// Provides: {"impl_57"}
// Dependencies: {}
impl < C > Neg for AffinePoint < C > where C : PrimeCurveParams , { type Output = Self ; # [inline] fn neg (self) -> Self { AffinePoint { x : self . x , y : - self . y , infinity : self . infinity , } } }
};
}
