// Generated macro for IncreasingUniform (struct)
macro_rules! Depcrate_seq_increasing_uniformIncreasingUniform {
() => {
// Module: crate::seq::increasing_uniform
// Provides: {"IncreasingUniform"}
// Dependencies: {}
# [doc = " Similar to a Uniform distribution,"] # [doc = " but after returning a number in the range [0,n], n is increased by 1."] pub (crate) struct IncreasingUniform < R : RngCore > { pub rng : R , n : u32 , chunk : u32 , chunk_remaining : u8 , }
};
}
