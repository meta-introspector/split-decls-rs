// Generated macro for impl_12 (impl)
macro_rules! Depcrate_byte_recordimpl_12 {
() => {
// Module: crate::byte_record
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < [T] > for ByteRecord { fn eq (& self , other : & [T]) -> bool { self . iter_eq (other) } }
};
}
