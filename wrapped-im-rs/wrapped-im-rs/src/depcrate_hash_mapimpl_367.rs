// Generated macro for impl_367 (impl)
macro_rules! Depcrate_hash_mapimpl_367 {
() => {
// Module: crate::hash::map
// Provides: {"impl_367"}
// Dependencies: {}
impl < 'a , K , V , S > From < & 'a Vec < (K , V) > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : & 'a Vec < (K , V) >) -> Self { m . iter () . cloned () . collect () } }
};
}
