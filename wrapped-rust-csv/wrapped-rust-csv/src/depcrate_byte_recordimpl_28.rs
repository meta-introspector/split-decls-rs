// Generated macro for impl_28 (impl)
macro_rules! Depcrate_byte_recordimpl_28 {
() => {
// Module: crate::byte_record
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : AsRef < [u8] > > Extend < T > for ByteRecord { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for x in iter { self . push_field (x . as_ref ()) ; } } }
};
}
