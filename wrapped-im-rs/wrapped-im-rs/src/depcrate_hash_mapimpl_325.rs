// Generated macro for impl_325 (impl)
macro_rules! Depcrate_hash_mapimpl_325 {
() => {
// Module: crate::hash::map
// Provides: {"impl_325"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K , V , S > PartialEq for HashMap < K , V , S > where K : Hash + Eq , V : Eq , S : BuildHasher , { fn eq (& self , other : & Self) -> bool { if PoolRef :: ptr_eq (& self . root , & other . root) { return true ; } self . test_eq (other) } }
};
}
