// Generated macro for impl_192 (impl)
macro_rules! Depcrate_ord_mapimpl_192 {
() => {
// Module: crate::ord::map
// Provides: {"impl_192"}
// Dependencies: {}
impl < K , V > Sum for OrdMap < K , V > where K : Ord + Clone , V : Clone , { fn sum < I > (it : I) -> Self where I : Iterator < Item = Self > , { it . fold (Self :: default () , | a , b | a + b) } }
};
}
