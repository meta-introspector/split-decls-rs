// Generated macro for impl_434 (impl)
macro_rules! Depcrate_hash_setimpl_434 {
() => {
// Module: crate::hash::set
// Provides: {"impl_434"}
// Dependencies: {}
impl < 'a , A , S > From < & 'a BTreeSet < A > > for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn from (btree_set : & BTreeSet < A >) -> Self { btree_set . iter () . cloned () . collect () } }
};
}
