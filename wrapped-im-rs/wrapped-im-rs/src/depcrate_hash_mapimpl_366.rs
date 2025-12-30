// Generated macro for impl_366 (impl)
macro_rules! Depcrate_hash_mapimpl_366 {
() => {
// Module: crate::hash::map
// Provides: {"impl_366"}
// Dependencies: {}
impl < K , V , S > From < Vec < (K , V) > > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from (m : Vec < (K , V) >) -> Self { m . into_iter () . collect () } }
};
}
