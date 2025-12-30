// Generated macro for impl_403 (impl)
macro_rules! Depcrate_hash_setimpl_403 {
() => {
// Module: crate::hash::set
// Provides: {"impl_403"}
// Dependencies: {}
impl < A , S > Ord for HashSet < A , S > where A : Hash + Eq + Clone + Ord , S : BuildHasher + Default , { fn cmp (& self , other : & Self) -> Ordering { if Ref :: ptr_eq (& self . hasher , & other . hasher) { return self . iter () . cmp (other . iter ()) ; } self . iter () . cmp (other . iter ()) } }
};
}
