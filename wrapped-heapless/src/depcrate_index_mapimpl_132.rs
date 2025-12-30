// Generated macro for impl_132 (impl)
macro_rules! Depcrate_index_mapimpl_132 {
() => {
// Module: crate::index_map
// Provides: {"impl_132"}
// Dependencies: {}
impl < K , V , S , const N : usize > Clone for IndexMap < K , V , S , N > where K : Clone , V : Clone , S : Clone , { fn clone (& self) -> Self { Self { core : self . core . clone () , build_hasher : self . build_hasher . clone () , } } }
};
}
