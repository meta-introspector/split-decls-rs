// Generated macro for impl_413 (impl)
macro_rules! Depcrate_hash_setimpl_413 {
() => {
// Module: crate::hash::set
// Provides: {"impl_413"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A , S > Debug for HashSet < A , S > where A : Hash + Eq + Debug , S : BuildHasher , { default fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
