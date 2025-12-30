// Generated macro for impl_34 (impl)
macro_rules! Depcrate_mapimpl_34 {
() => {
// Module: crate::map
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , S > LiteMap < K , V , S > where S : StoreIterableMut < 'a , K , V > , { # [doc = " Produce an ordered mutable iterator over key-value pairs"] pub fn iter_mut (& 'a mut self) -> impl DoubleEndedIterator < Item = (& 'a K , & 'a mut V) > { self . values . lm_iter_mut () } }
};
}
