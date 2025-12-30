// Generated macro for impl_147 (impl)
macro_rules! Depcrate_linked_hash_setimpl_147 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_147"}
// Dependencies: {}
impl < T , S > Hash for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { for e in self { e . hash (state) ; } } }
};
}
