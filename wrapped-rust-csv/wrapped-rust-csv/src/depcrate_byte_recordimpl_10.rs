// Generated macro for impl_10 (impl)
macro_rules! Depcrate_byte_recordimpl_10 {
() => {
// Module: crate::byte_record
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < Vec < T > > for ByteRecord { fn eq (& self , other : & Vec < T >) -> bool { self . iter_eq (other) } }
};
}
