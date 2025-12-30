// Generated macro for impl_404 (impl)
macro_rules! Depcrate_hash_setimpl_404 {
() => {
// Module: crate::hash::set
// Provides: {"impl_404"}
// Dependencies: {}
impl < A , S > Hash for HashSet < A , S > where A : Hash + Eq , S : BuildHasher + Default , { fn hash < H > (& self , state : & mut H) where H : Hasher , { for i in self . iter () { i . hash (state) ; } } }
};
}
