// Generated macro for impl_368 (impl)
macro_rules! Depcrate_hash_mapimpl_368 {
() => {
// Module: crate::hash::map
// Provides: {"impl_368"}
// Dependencies: {}
impl < K , V , S > From < collections :: HashMap < K , V > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : collections :: HashMap < K , V >) -> Self { m . into_iter () . collect () } }
};
}
