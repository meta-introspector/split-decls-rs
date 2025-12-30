// Generated macro for impl_16 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_16 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_16"}
// Dependencies: {}
impl < K : Hash + Eq , V : PartialEq , S : BuildHasher > PartialEq for LinkedHashMap < K , V , S > { # [inline] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
};
}
