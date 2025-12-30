// Generated macro for impl_211 (impl)
macro_rules! Depcrate_linear_mapimpl_211 {
() => {
// Module: crate::linear_map
// Provides: {"impl_211"}
// Dependencies: {}
impl < K , V , const N : usize > Clone for LinearMap < K , V , N > where K : Eq + Clone , V : Clone , { fn clone (& self) -> Self { Self { buffer : self . buffer . clone () , } } }
};
}
