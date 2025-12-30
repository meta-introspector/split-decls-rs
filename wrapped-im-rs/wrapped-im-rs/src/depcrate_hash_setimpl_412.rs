// Generated macro for impl_412 (impl)
macro_rules! Depcrate_hash_setimpl_412 {
() => {
// Module: crate::hash::set
// Provides: {"impl_412"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < A , S > Debug for HashSet < A , S > where A : Hash + Eq + Debug , S : BuildHasher , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
