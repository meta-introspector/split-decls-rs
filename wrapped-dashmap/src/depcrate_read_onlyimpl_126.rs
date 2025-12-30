// Generated macro for impl_126 (impl)
macro_rules! Depcrate_read_onlyimpl_126 {
() => {
// Module: crate::read_only
// Provides: {"impl_126"}
// Dependencies: {}
impl < K : Eq + Hash + Clone , V : Clone , S : Clone > Clone for ReadOnlyView < K , V , S > { fn clone (& self) -> Self { Self { map : self . map . clone () , } } }
};
}
