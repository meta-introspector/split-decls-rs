// Generated macro for impl_266 (impl)
macro_rules! Depcrate_bytes_mutimpl_266 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_266"}
// Dependencies: {}
impl Extend < u8 > for BytesMut { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = u8 > , { let iter = iter . into_iter () ; let (lower , _) = iter . size_hint () ; self . reserve (lower) ; for b in iter { self . put_u8 (b) ; } } }
};
}
