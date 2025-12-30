// Generated macro for impl_8 (impl)
macro_rules! Depcrate_recoveryimpl_8 {
() => {
// Module: crate::recovery
// Provides: {"impl_8"}
// Dependencies: {}
impl TryFrom < u8 > for RecoveryId { type Error = Error ; fn try_from (byte : u8) -> Result < Self > { Self :: from_byte (byte) . ok_or_else (Error :: new) } }
};
}
