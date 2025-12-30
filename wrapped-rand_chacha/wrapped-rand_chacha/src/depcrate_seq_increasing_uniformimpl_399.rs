// Generated macro for impl_399 (impl)
macro_rules! Depcrate_seq_increasing_uniformimpl_399 {
() => {
// Module: crate::seq::increasing_uniform
// Provides: {"impl_399"}
// Dependencies: {}
impl < R : RngCore > IncreasingUniform < R > { # [doc = " Create a dice roller."] # [doc = " The next item returned will be a random number in the range [0,n]"] pub fn new (rng : R , n : u32) -> Self { let chunk_remaining = if n == 0 { 1 } else { 0 } ; Self { rng , n , chunk : 0 , chunk_remaining , } } # [doc = " Returns a number in [0,n] and increments n by 1."] # [doc = " Generates new random bits as needed"] # [doc = " Panics if `n >= u32::MAX`"] # [inline] pub fn next_index (& mut self) -> usize { let next_n = self . n + 1 ; let next_chunk_remaining = self . chunk_remaining . checked_sub (1) . unwrap_or_else (| | { let (bound , remaining) = calculate_bound_u32 (next_n) ; self . chunk = self . rng . random_range (.. bound) ; remaining - 1 }) ; let result = if next_chunk_remaining == 0 { self . chunk as usize } else { let r = self . chunk % next_n ; self . chunk /= next_n ; r as usize } ; self . chunk_remaining = next_chunk_remaining ; self . n = next_n ; result } }
};
}
