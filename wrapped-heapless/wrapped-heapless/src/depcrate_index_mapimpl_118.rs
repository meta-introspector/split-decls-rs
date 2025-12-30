// Generated macro for impl_118 (impl)
macro_rules! Depcrate_index_mapimpl_118 {
() => {
// Module: crate::index_map
// Provides: {"impl_118"}
// Dependencies: {}
impl < K , V , const N : usize > Clone for CoreMap < K , V , N > where K : Clone , V : Clone , { fn clone (& self) -> Self { Self { entries : self . entries . clone () , indices : self . indices , } } }
};
}
