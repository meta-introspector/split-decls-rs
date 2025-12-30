// Generated macro for impl_11 (impl)
macro_rules! Depcrate_byte_recordimpl_11 {
() => {
// Module: crate::byte_record
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : AsRef < [u8] > > PartialEq < Vec < T > > for & ByteRecord { fn eq (& self , other : & Vec < T >) -> bool { self . iter_eq (other) } }
};
}
