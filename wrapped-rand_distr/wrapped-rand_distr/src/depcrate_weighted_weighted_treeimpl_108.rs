// Generated macro for impl_108 (impl)
macro_rules! Depcrate_weighted_weighted_treeimpl_108 {
() => {
// Module: crate::weighted::weighted_tree
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Samples a randomly selected index from the weighted distribution."] # [doc = ""] # [doc = " Caution: This method panics if there are no elements or all weights are zero. However,"] # [doc = " it is guaranteed that this method will not panic if a call to [`WeightedTreeIndex::is_valid`]"] # [doc = " returns `true`."] impl < W : Clone + PartialEq + PartialOrd + SampleUniform + SubAssign < W > + Weight > Distribution < usize > for WeightedTreeIndex < W > { # [track_caller] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> usize { self . try_sample (rng) . unwrap () } }
};
}
