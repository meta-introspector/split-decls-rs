// Generated macro for impl_406 (impl)
macro_rules! Depcrate_hash_setimpl_406 {
() => {
// Module: crate::hash::set
// Provides: {"impl_406"}
// Dependencies: {}
impl < A , S > Add for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher , { type Output = HashSet < A , S > ; fn add (self , other : Self) -> Self :: Output { self . union (other) } }
};
}
