// Generated macro for impl_87 (impl)
macro_rules! Depcrate_mapimpl_87 {
() => {
// Module: crate::map
// Provides: {"impl_87"}
// Dependencies: {}
impl < K , V , S > Clone for IndexMap < K , V , S > where K : Clone , V : Clone , S : Clone , { fn clone (& self) -> Self { IndexMap { core : self . core . clone () , hash_builder : self . hash_builder . clone () , } } fn clone_from (& mut self , other : & Self) { self . core . clone_from (& other . core) ; self . hash_builder . clone_from (& other . hash_builder) ; } }
};
}
