// Generated macro for impl_121 (impl)
macro_rules! Depcrate_devimpl_121 {
() => {
// Module: crate::dev
// Provides: {"impl_121"}
// Dependencies: {}
impl TryFrom < ProjectivePoint > for NonIdentity < ProjectivePoint > { type Error = Error ; fn try_from (point : ProjectivePoint) -> Result < Self > { NonIdentity :: new (point) . into_option () . ok_or (Error) } }
};
}
