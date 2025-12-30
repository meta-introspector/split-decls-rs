// Generated macro for impl_408 (impl)
macro_rules! Depcrate_hash_setimpl_408 {
() => {
// Module: crate::hash::set
// Provides: {"impl_408"}
// Dependencies: {}
impl < 'a , A , S > Add for & 'a HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher , { type Output = HashSet < A , S > ; fn add (self , other : Self) -> Self :: Output { self . clone () . union (other . clone ()) } }
};
}
