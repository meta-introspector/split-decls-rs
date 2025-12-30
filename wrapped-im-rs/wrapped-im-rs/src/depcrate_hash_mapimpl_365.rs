// Generated macro for impl_365 (impl)
macro_rules! Depcrate_hash_mapimpl_365 {
() => {
// Module: crate::hash::map
// Provides: {"impl_365"}
// Dependencies: {}
impl < 'a , K , V , S > From < & 'a [(K , V)] > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : & 'a [(K , V)]) -> Self { m . iter () . cloned () . collect () } }
};
}
