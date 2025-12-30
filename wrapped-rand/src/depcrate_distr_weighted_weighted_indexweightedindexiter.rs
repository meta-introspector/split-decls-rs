// Generated macro for WeightedIndexIter (struct)
macro_rules! Depcrate_distr_weighted_weighted_indexWeightedIndexIter {
() => {
// Module: crate::distr::weighted::weighted_index
// Provides: {"WeightedIndexIter"}
// Dependencies: {}
# [doc = " A lazy-loading iterator over the weights of a `WeightedIndex` distribution."] # [doc = " This is returned by [`WeightedIndex::weights`]."] pub struct WeightedIndexIter < 'a , X : SampleUniform + PartialOrd > { weighted_index : & 'a WeightedIndex < X > , index : usize , }
};
}
