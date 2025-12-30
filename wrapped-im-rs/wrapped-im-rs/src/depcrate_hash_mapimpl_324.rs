// Generated macro for impl_324 (impl)
macro_rules! Depcrate_hash_mapimpl_324 {
() => {
// Module: crate::hash::map
// Provides: {"impl_324"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K , V , S > PartialEq for HashMap < K , V , S > where K : Hash + Eq , V : PartialEq , S : BuildHasher , { default fn eq (& self , other : & Self) -> bool { self . test_eq (other) } }
};
}
