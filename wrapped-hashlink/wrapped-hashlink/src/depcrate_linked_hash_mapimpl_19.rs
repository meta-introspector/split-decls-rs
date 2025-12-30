// Generated macro for impl_19 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_19 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_19"}
// Dependencies: {}
impl < K : Hash + Eq + Ord , V : Ord , S : BuildHasher > Ord for LinkedHashMap < K , V , S > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other) } }
};
}
