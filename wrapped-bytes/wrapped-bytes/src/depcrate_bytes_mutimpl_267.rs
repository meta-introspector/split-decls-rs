// Generated macro for impl_267 (impl)
macro_rules! Depcrate_bytes_mutimpl_267 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'a > Extend < & 'a u8 > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = & 'a u8 > , { self . extend (iter . into_iter () . copied ()) } }
};
}
