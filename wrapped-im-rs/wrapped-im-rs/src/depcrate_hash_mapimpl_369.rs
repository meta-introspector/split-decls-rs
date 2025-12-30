// Generated macro for impl_369 (impl)
macro_rules! Depcrate_hash_mapimpl_369 {
() => {
// Module: crate::hash::map
// Provides: {"impl_369"}
// Dependencies: {}
impl < 'a , K , V , S > From < & 'a collections :: HashMap < K , V > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : & 'a collections :: HashMap < K , V >) -> Self { m . iter () . map (| (k , v) | (k . clone () , v . clone ())) . collect () } }
};
}
