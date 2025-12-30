// Generated macro for impl_327 (impl)
macro_rules! Depcrate_hash_mapimpl_327 {
() => {
// Module: crate::hash::map
// Provides: {"impl_327"}
// Dependencies: {}
impl < K , V , S > PartialOrd for HashMap < K , V , S > where K : Hash + Eq + Clone + PartialOrd , V : PartialOrd + Clone , S : BuildHasher , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if Ref :: ptr_eq (& self . hasher , & other . hasher) { return self . iter () . partial_cmp (other . iter ()) ; } self . iter () . partial_cmp (other . iter ()) } }
};
}
