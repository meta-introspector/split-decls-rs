// Generated macro for impl_407 (impl)
macro_rules! Depcrate_hash_setimpl_407 {
() => {
// Module: crate::hash::set
// Provides: {"impl_407"}
// Dependencies: {}
impl < A , S > Mul for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher , { type Output = HashSet < A , S > ; fn mul (self , other : Self) -> Self :: Output { self . intersection (other) } }
};
}
