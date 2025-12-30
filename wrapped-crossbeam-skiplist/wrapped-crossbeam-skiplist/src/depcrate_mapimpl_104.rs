// Generated macro for impl_104 (impl)
macro_rules! Depcrate_mapimpl_104 {
() => {
// Module: crate::map
// Provides: {"impl_104"}
// Dependencies: {}
impl < K , V > Drop for Iter < '_ , K , V > { fn drop (& mut self) { let guard = & epoch :: pin () ; self . inner . drop_impl (guard) ; } }
};
}
