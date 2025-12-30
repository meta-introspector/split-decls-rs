// Generated macro for impl_265 (impl)
macro_rules! Depcrate_bytes_mutimpl_265 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a BytesMut { type Item = & 'a u8 ; type IntoIter = core :: slice :: Iter < 'a , u8 > ; fn into_iter (self) -> Self :: IntoIter { self . as_ref () . iter () } }
};
}
