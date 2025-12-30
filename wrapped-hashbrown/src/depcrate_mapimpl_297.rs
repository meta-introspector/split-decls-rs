// Generated macro for impl_297 (impl)
macro_rules! Depcrate_mapimpl_297 {
() => {
// Module: crate::map
// Provides: {"impl_297"}
// Dependencies: {}
impl < K : Clone , V : Clone , S : Clone , A : Allocator + Clone > Clone for HashMap < K , V , S , A > { fn clone (& self) -> Self { HashMap { hash_builder : self . hash_builder . clone () , table : self . table . clone () , } } fn clone_from (& mut self , source : & Self) { self . table . clone_from (& source . table) ; self . hash_builder . clone_from (& source . hash_builder) ; } }
};
}
