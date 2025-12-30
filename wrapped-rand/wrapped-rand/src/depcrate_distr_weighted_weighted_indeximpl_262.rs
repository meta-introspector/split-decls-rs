// Generated macro for impl_262 (impl)
macro_rules! Depcrate_distr_weighted_weighted_indeximpl_262 {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"impl_262"}
// Dependencies: {}
impl < X > Debug for WeightedIndexIter < '_ , X > where X : SampleUniform + PartialOrd + Debug , X :: Sampler : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WeightedIndexIter") . field ("weighted_index" , & self . weighted_index) . field ("index" , & self . index) . finish () } }
};
}
