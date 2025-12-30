// Generated macro for impl_410 (impl)
macro_rules! Depcrate_hash_setimpl_410 {
() => {
// Module: crate::hash::set
// Provides: {"impl_410"}
// Dependencies: {}
impl < A , S > Sum for HashSet < A , S > where A : Hash + Eq + Clone , S : BuildHasher + Default , { fn sum < I > (it : I) -> Self where I : Iterator < Item = Self > , { it . fold (Self :: default () , | a , b | a + b) } }
};
}
