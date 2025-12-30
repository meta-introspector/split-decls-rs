// Generated macro for impl_270 (impl)
macro_rules! Depcrate_bytes_mutimpl_270 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a u8 > for BytesMut { fn from_iter < T : IntoIterator < Item = & 'a u8 > > (into_iter : T) -> Self { BytesMut :: from_iter (into_iter . into_iter () . copied ()) } }
};
}
