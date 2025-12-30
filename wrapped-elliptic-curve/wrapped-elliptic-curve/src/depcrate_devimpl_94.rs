// Generated macro for impl_94 (impl)
macro_rules! Depcrate_devimpl_94 {
() => {
// Module: crate::dev
// Provides: {"impl_94"}
// Dependencies: {}
impl TryFrom < Scalar > for NonZeroScalar { type Error = Error ; fn try_from (scalar : Scalar) -> Result < Self > { NonZeroScalar :: new (scalar) . into_option () . ok_or (Error) } }
};
}
