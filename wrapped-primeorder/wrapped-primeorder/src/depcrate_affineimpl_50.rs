// Generated macro for impl_50 (impl)
macro_rules! Depcrate_affineimpl_50 {
() => {
// Module: crate::affine
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " The constant-time alternative is available at [`NonIdentity::new()`]."] impl < C > TryFrom < AffinePoint < C > > for NonIdentity < AffinePoint < C > > where C : PrimeCurveParams , { type Error = Error ; fn try_from (affine_point : AffinePoint < C >) -> Result < Self > { NonIdentity :: new (affine_point) . into_option () . ok_or (Error) } }
};
}
