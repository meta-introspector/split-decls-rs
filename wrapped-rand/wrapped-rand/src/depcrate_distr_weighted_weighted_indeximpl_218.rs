// Generated macro for impl_218 (impl)
macro_rules! Depcrate_distr_weighted_weighted_indeximpl_218 {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"impl_218"}
// Dependencies: {}
impl < X > Iterator for WeightedIndexIter < '_ , X > where X : for < 'b > core :: ops :: SubAssign < & 'b X > + SampleUniform + PartialOrd + Clone , { type Item = X ; fn next (& mut self) -> Option < Self :: Item > { match self . weighted_index . weight (self . index) { None => None , Some (weight) => { self . index += 1 ; Some (weight) } } } }
};
}
