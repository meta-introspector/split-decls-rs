// Generated macro for impl_82 (impl)
macro_rules! Depcrate_inputimpl_82 {
() => {
// Module: crate::input
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Env { type Item = (& 'a String , & 'a String) ; type IntoIter = std :: collections :: hash_map :: Iter < 'a , String , String > ; fn into_iter (self) -> Self :: IntoIter { self . entries . iter () } }
};
}
