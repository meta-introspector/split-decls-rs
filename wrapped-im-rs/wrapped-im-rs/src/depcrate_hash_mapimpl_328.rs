// Generated macro for impl_328 (impl)
macro_rules! Depcrate_hash_mapimpl_328 {
() => {
// Module: crate::hash::map
// Provides: {"impl_328"}
// Dependencies: {}
impl < K , V , S > Ord for HashMap < K , V , S > where K : Hash + Eq + Ord + Clone , V : Ord + Clone , S : BuildHasher , { fn cmp (& self , other : & Self) -> Ordering { if Ref :: ptr_eq (& self . hasher , & other . hasher) { return self . iter () . cmp (other . iter ()) ; } self . iter () . cmp (other . iter ()) } }
};
}
