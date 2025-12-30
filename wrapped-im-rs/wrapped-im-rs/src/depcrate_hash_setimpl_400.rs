// Generated macro for impl_400 (impl)
macro_rules! Depcrate_hash_setimpl_400 {
() => {
// Module: crate::hash::set
// Provides: {"impl_400"}
// Dependencies: {}
impl < A , S > PartialEq for HashSet < A , S > where A : Hash + Eq , S : BuildHasher + Default , { fn eq (& self , other : & Self) -> bool { self . test_eq (other) } }
};
}
