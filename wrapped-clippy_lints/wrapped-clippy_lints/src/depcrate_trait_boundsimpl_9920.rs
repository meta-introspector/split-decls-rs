// Generated macro for impl_9920 (impl)
macro_rules! Depcrate_trait_boundsimpl_9920 {
() => {
// Module: crate::trait_bounds
// Provides: {"impl_9920"}
// Dependencies: {}
impl Hash for ComparableTraitRef < '_ , '_ > { fn hash < H : Hasher > (& self , state : & mut H) { let mut s = SpanlessHash :: new (self . cx) . paths_by_resolution () ; s . hash_path (self . trait_ref . path) ; s . hash_modifiers (self . modifiers) ; state . write_u64 (s . finish ()) ; } }
};
}
