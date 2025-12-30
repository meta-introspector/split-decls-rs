// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for & 'a ObjectIdentifierRef { type Error = Error ; fn try_from (ber_bytes : & 'a [u8]) -> Result < Self > { ObjectIdentifierRef :: from_bytes (ber_bytes) } }
};
}
