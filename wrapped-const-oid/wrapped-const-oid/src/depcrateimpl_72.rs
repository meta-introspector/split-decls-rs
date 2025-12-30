// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl TryFrom < & [u8] > for ObjectIdentifier { type Error = Error ; fn try_from (ber_bytes : & [u8]) -> Result < Self > { Self :: from_bytes (ber_bytes) } }
};
}
