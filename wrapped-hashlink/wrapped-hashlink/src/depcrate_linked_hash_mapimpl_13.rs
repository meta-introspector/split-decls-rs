// Generated macro for impl_13 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_13 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_13"}
// Dependencies: {}
impl < K , V , S > Default for LinkedHashMap < K , V , S > where S : Default , { # [inline] fn default () -> Self { Self :: with_hasher (S :: default ()) } }
};
}
