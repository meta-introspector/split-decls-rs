// Generated macro for impl_337 (impl)
macro_rules! Depcrate_hash_mapimpl_337 {
() => {
// Module: crate::hash::map
// Provides: {"impl_337"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < K , V , S > Debug for HashMap < K , V , S > where K : Hash + Eq + Debug , V : Debug , S : BuildHasher , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { let mut d = f . debug_map () ; for (k , v) in self { d . entry (k , v) ; } d . finish () } }
};
}
