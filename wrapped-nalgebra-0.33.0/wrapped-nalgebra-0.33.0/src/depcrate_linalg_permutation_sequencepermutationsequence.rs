// Generated macro for PermutationSequence (struct)
macro_rules! Depcrate_linalg_permutation_sequencePermutationSequence {
() => {
// Module: crate::linalg::permutation_sequence
// Provides: {"PermutationSequence"}
// Dependencies: {}
# [doc = " A sequence of row or column permutations."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D>,
         OVector<(usize, usize), D>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D>,
         OVector<(usize, usize), D>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct PermutationSequence < D : Dim > where DefaultAllocator : Allocator < D > , { len : usize , ipiv : OVector < (usize , usize) , D > , }
};
}
