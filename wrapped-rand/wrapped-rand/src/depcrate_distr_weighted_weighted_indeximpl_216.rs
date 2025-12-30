// Generated macro for impl_216 (impl)
macro_rules! Depcrate_distr_weighted_weighted_indeximpl_216 {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"impl_216"}
// Dependencies: {}
impl < X > Debug for WeightedIndexIter < '_ , X > where X : SampleUniform + PartialOrd + Debug , X :: Sampler : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WeightedIndexIter") . field ("weighted_index" , & self . weighted_index) . field ("index" , & self . index) . finish () } }
};
}
