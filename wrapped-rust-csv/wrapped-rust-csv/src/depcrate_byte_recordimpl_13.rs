// Generated macro for impl_13 (impl)
macro_rules! Depcrate_byte_recordimpl_13 {
() => {
// Module: crate::byte_record
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < [T] > for & ByteRecord { fn eq (& self , other : & [T]) -> bool { self . iter_eq (other) } }
};
}
