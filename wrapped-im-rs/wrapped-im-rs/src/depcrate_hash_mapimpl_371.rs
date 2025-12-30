// Generated macro for impl_371 (impl)
macro_rules! Depcrate_hash_mapimpl_371 {
() => {
// Module: crate::hash::map
// Provides: {"impl_371"}
// Dependencies: {}
impl < 'a , K , V , S > From < & 'a collections :: BTreeMap < K , V > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : & 'a collections :: BTreeMap < K , V >) -> Self { m . iter () . map (| (k , v) | (k . clone () , v . clone ())) . collect () } }
};
}
