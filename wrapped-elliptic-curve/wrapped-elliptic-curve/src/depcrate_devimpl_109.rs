// Generated macro for impl_109 (impl)
macro_rules! Depcrate_devimpl_109 {
() => {
// Module: crate::dev
// Provides: {"impl_109"}
// Dependencies: {}
impl TryFrom < AffinePoint > for NonIdentity < AffinePoint > { type Error = Error ; fn try_from (affine : AffinePoint) -> Result < Self > { NonIdentity :: new (affine) . into_option () . ok_or (Error) } }
};
}
