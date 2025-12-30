// Generated macro for impl_43 (impl)
macro_rules! Depcrate_buf_chainimpl_43 {
() => {
// Module: crate::buf::chain
// Provides: {"impl_43"}
// Dependencies: {}
impl < T , U > IntoIterator for Chain < T , U > where T : Buf , U : Buf , { type Item = u8 ; type IntoIter = IntoIter < Chain < T , U > > ; fn into_iter (self) -> Self :: IntoIter { IntoIter :: new (self) } }
};
}
