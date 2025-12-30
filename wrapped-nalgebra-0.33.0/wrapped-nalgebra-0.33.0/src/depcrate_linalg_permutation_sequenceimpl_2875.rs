// Generated macro for impl_2875 (impl)
macro_rules! Depcrate_linalg_permutation_sequenceimpl_2875 {
() => {
// Module: crate::linalg::permutation_sequence
// Provides: {"impl_2875"}
// Dependencies: {}
impl < D : DimName > PermutationSequence < D > where DefaultAllocator : Allocator < D > , { # [doc = " Creates a new statically-allocated sequence of `D` identity permutations."] # [inline] pub fn identity () -> Self { Self :: identity_generic (D :: name ()) } }
};
}
