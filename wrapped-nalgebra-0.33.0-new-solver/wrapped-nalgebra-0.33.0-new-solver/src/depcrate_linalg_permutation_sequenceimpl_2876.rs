// Generated macro for impl_2876 (impl)
macro_rules! Depcrate_linalg_permutation_sequenceimpl_2876 {
() => {
// Module: crate::linalg::permutation_sequence
// Provides: {"impl_2876"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl PermutationSequence < Dyn > where DefaultAllocator : Allocator < Dyn > , { # [doc = " Creates a new dynamically-allocated sequence of `n` identity permutations."] # [inline] pub fn identity (n : usize) -> Self { Self :: identity_generic (Dyn (n)) } }
};
}
