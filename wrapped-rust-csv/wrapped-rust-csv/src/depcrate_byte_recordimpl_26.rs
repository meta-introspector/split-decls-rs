// Generated macro for impl_26 (impl)
macro_rules! Depcrate_byte_recordimpl_26 {
() => {
// Module: crate::byte_record
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , T : AsRef < [u8] > > From < & 'a [T] > for ByteRecord { # [inline] fn from (xs : & 'a [T]) -> ByteRecord { ByteRecord :: from_iter (xs) } }
};
}
