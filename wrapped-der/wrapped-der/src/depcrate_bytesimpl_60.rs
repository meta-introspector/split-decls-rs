// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bytesimpl_60 {
() => {
// Module: crate::bytes
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for & 'a BytesRef { type Error = Error ; fn try_from (slice : & 'a [u8]) -> Result < Self > { BytesRef :: new (slice) } }
};
}
