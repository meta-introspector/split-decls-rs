// Generated macro for impl_81 (impl)
macro_rules! Depcrate_inputimpl_81 {
() => {
// Module: crate::input
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Env { type Item = (& 'a String , & 'a String) ; type IntoIter = std :: collections :: hash_map :: Iter < 'a , String , String > ; fn into_iter (self) -> Self :: IntoIter { self . entries . iter () } }
};
}
