// Generated macro for impl_269 (impl)
macro_rules! Depcrate_bytes_mutimpl_269 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_269"}
// Dependencies: {}
impl FromIterator < u8 > for BytesMut { fn from_iter < T : IntoIterator < Item = u8 > > (into_iter : T) -> Self { BytesMut :: from_vec (Vec :: from_iter (into_iter)) } }
};
}
