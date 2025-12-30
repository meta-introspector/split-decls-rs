// Generated macro for impl_370 (impl)
macro_rules! Depcrate_hash_mapimpl_370 {
() => {
// Module: crate::hash::map
// Provides: {"impl_370"}
// Dependencies: {}
impl < K , V , S > From < collections :: BTreeMap < K , V > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : collections :: BTreeMap < K , V >) -> Self { m . into_iter () . collect () } }
};
}
