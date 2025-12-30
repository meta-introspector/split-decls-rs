// Generated macro for impl_53 (impl)
macro_rules! Depcrate_affineimpl_53 {
() => {
// Module: crate::affine
// Provides: {"impl_53"}
// Dependencies: {}
impl < C > TryFrom < AffinePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (affine_point : AffinePoint < C >) -> Result < PublicKey < C > > { PublicKey :: from_affine (affine_point) } }
};
}
