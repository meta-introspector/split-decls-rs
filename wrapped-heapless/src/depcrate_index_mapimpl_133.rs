// Generated macro for impl_133 (impl)
macro_rules! Depcrate_index_mapimpl_133 {
() => {
// Module: crate::index_map
// Provides: {"impl_133"}
// Dependencies: {}
impl < K , V , S , const N : usize > fmt :: Debug for IndexMap < K , V , S , N > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
