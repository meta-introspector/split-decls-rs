// Generated macro for impl_251 (impl)
macro_rules! Depcrate_bytes_mutimpl_251 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for BytesMut { fn from (src : & 'a [u8]) -> BytesMut { BytesMut :: from_vec (src . to_vec ()) } }
};
}
