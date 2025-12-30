// Generated macro for impl_409 (impl)
macro_rules! Depcrate_hash_setimpl_409 {
() => {
// Module: crate::hash::set
// Provides: {"impl_409"}
// Dependencies: {}
impl < 'a , A , S > Mul for & 'a HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher , { type Output = HashSet < A , S > ; fn mul (self , other : Self) -> Self :: Output { self . clone () . intersection (other . clone ()) } }
};
}
