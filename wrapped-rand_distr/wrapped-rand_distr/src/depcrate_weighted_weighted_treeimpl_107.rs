// Generated macro for impl_107 (impl)
macro_rules! Depcrate_weighted_weighted_treeimpl_107 {
() => {
// Module: crate::weighted::weighted_tree
// Provides: {"impl_107"}
// Dependencies: {}
impl < W : Clone + PartialEq + PartialOrd + SampleUniform + SubAssign < W > + Weight > WeightedTreeIndex < W > { # [doc = " Samples a randomly selected index from the weighted distribution."] # [doc = ""] # [doc = " Returns an error if there are no elements or all weights are zero. This"] # [doc = " is unlike [`Distribution::sample`], which panics in those cases."] pub fn try_sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Result < usize , Error > { let total_weight = self . subtotals . first () . cloned () . unwrap_or (W :: ZERO) ; if total_weight == W :: ZERO { return Err (Error :: InsufficientNonZero) ; } let mut target_weight = rng . random_range (W :: ZERO .. total_weight) ; let mut index = 0 ; loop { let left_index = 2 * index + 1 ; let left_subtotal = self . subtotal (left_index) ; if target_weight < left_subtotal { index = left_index ; continue ; } target_weight -= left_subtotal ; let right_index = 2 * index + 2 ; let right_subtotal = self . subtotal (right_index) ; if target_weight < right_subtotal { index = right_index ; continue ; } target_weight -= right_subtotal ; break ; } assert ! (target_weight >= W :: ZERO) ; assert ! (target_weight < self . get (index)) ; Ok (index) } }
};
}
