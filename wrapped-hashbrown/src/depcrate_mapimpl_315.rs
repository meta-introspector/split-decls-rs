// Generated macro for impl_315 (impl)
macro_rules! Depcrate_mapimpl_315 {
() => {
// Module: crate::map
// Provides: {"impl_315"}
// Dependencies: {}
impl < K , V > Clone for Iter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
