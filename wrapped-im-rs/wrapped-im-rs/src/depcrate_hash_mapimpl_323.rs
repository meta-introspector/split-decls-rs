// Generated macro for impl_323 (impl)
macro_rules! Depcrate_hash_mapimpl_323 {
() => {
// Module: crate::hash::map
// Provides: {"impl_323"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < K , V , S > PartialEq for HashMap < K , V , S > where K : Hash + Eq , V : PartialEq , S : BuildHasher , { fn eq (& self , other : & Self) -> bool { self . test_eq (other) } }
};
}
