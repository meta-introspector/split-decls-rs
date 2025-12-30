// Generated macro for impl_129 (impl)
macro_rules! Depcrate_bytesimpl_129 {
() => {
// Module: crate::bytes
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Bytes { type Item = & 'a u8 ; type IntoIter = core :: slice :: Iter < 'a , u8 > ; fn into_iter (self) -> Self :: IntoIter { self . as_slice () . iter () } }
};
}
