// Generated macro for impl_268 (impl)
macro_rules! Depcrate_bytes_mutimpl_268 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_268"}
// Dependencies: {}
impl Extend < Bytes > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = Bytes > , { for bytes in iter { self . extend_from_slice (& bytes) } } }
};
}
