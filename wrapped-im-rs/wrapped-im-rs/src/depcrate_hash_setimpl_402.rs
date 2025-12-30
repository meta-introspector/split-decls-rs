// Generated macro for impl_402 (impl)
macro_rules! Depcrate_hash_setimpl_402 {
() => {
// Module: crate::hash::set
// Provides: {"impl_402"}
// Dependencies: {}
impl < A , S > PartialOrd for HashSet < A , S > where A : Hash + Eq + Clone + PartialOrd , S : BuildHasher + Default , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if Ref :: ptr_eq (& self . hasher , & other . hasher) { return self . iter () . partial_cmp (other . iter ()) ; } self . iter () . partial_cmp (other . iter ()) } }
};
}
