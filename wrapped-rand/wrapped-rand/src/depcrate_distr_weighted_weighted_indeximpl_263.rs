// Generated macro for impl_263 (impl)
macro_rules! Depcrate_distr_weighted_weighted_indeximpl_263 {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"impl_263"}
// Dependencies: {}
impl < X > Clone for WeightedIndexIter < '_ , X > where X : SampleUniform + PartialOrd , { fn clone (& self) -> Self { WeightedIndexIter { weighted_index : self . weighted_index , index : self . index , } } }
};
}
