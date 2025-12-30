// Generated macro for impl_20 (impl)
macro_rules! Depcrate_iterimpl_20 {
() => {
// Module: crate::iter
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'i , K : Clone + Hash + Eq , V : Clone > Clone for Iter < 'i , K , V > { fn clone (& self) -> Self { Iter { shards : self . shards . clone () , current : self . current . clone () , } } }
};
}
