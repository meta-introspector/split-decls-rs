// Generated macro for impl_25 (impl)
macro_rules! Depcrate_byte_recordimpl_25 {
() => {
// Module: crate::byte_record
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : AsRef < [u8] > > From < Vec < T > > for ByteRecord { # [inline] fn from (xs : Vec < T >) -> ByteRecord { ByteRecord :: from_iter (& xs) } }
};
}
