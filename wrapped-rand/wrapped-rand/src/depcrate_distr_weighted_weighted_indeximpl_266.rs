// Generated macro for impl_266 (impl)
macro_rules! Depcrate_distr_weighted_weighted_indeximpl_266 {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"impl_266"}
// Dependencies: {}
impl < X > Distribution < usize > for WeightedIndex < X > where X : SampleUniform + PartialOrd , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> usize { let chosen_weight = self . weight_distribution . sample (rng) ; self . cumulative_weights . partition_point (| w | w <= & chosen_weight) } }
};
}
