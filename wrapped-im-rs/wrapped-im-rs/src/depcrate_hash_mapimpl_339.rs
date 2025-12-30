// Generated macro for impl_339 (impl)
macro_rules! Depcrate_hash_mapimpl_339 {
() => {
// Module: crate::hash::map
// Provides: {"impl_339"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K , V , S > Debug for HashMap < K , V , S > where K : Hash + Eq + Ord + Debug , V : Debug , S : BuildHasher , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { let mut keys = collections :: BTreeSet :: new () ; keys . extend (self . keys ()) ; let mut d = f . debug_map () ; for key in keys { d . entry (key , & self [key]) ; } d . finish () } }
};
}
