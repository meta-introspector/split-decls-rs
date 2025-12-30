// Generated macro for impl_433 (impl)
macro_rules! Depcrate_hash_setimpl_433 {
() => {
// Module: crate::hash::set
// Provides: {"impl_433"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a collections :: HashSet < A > > for HashSet < A , S > where A : Eq + Hash + Clone , S : BuildHasher + Default , { fn from (hash_set : & collections :: HashSet < A >) -> Self { hash_set . iter () . cloned () . collect () } }
};
}
