// Generated macro for impl_414 (impl)
macro_rules! Depcrate_hash_setimpl_414 {
() => {
// Module: crate::hash::set
// Provides: {"impl_414"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A , S > Debug for HashSet < A , S > where A : Hash + Eq + Debug + Ord , S : BuildHasher , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
