// Generated macro for impl_102 (impl)
macro_rules! Depcrate_mapimpl_102 {
() => {
// Module: crate::map
// Provides: {"impl_102"}
// Dependencies: {}
impl < K , V , S > Default for IndexMap < K , V , S > where S : Default , { # [doc = " Return an empty [`IndexMap`]"] fn default () -> Self { Self :: with_capacity_and_hasher (0 , S :: default ()) } }
};
}
