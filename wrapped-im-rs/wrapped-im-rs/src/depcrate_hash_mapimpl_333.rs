// Generated macro for impl_333 (impl)
macro_rules! Depcrate_hash_mapimpl_333 {
() => {
// Module: crate::hash::map
// Provides: {"impl_333"}
// Dependencies: {}
impl < K , V , S > Sum for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn sum < I > (it : I) -> Self where I : Iterator < Item = Self > , { it . fold (Self :: default () , | a , b | a + b) } }
};
}
