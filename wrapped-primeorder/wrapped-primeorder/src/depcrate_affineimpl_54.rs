// Generated macro for impl_54 (impl)
macro_rules! Depcrate_affineimpl_54 {
() => {
// Module: crate::affine
// Provides: {"impl_54"}
// Dependencies: {}
impl < C > TryFrom < & AffinePoint < C > > for PublicKey < C > where C : PrimeCurveParams , { type Error = Error ; fn try_from (affine_point : & AffinePoint < C >) -> Result < PublicKey < C > > { PublicKey :: < C > :: try_from (* affine_point) } }
};
}
