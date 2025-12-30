// Generated macro for impl_43 (impl)
macro_rules! Depcrate_affineimpl_43 {
() => {
// Module: crate::affine
// Provides: {"impl_43"}
// Dependencies: {}
impl < C > From < & PublicKey < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (public_key : & PublicKey < C >) -> AffinePoint < C > { AffinePoint :: from (* public_key) } }
};
}
