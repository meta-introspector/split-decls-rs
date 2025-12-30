// Generated macro for rand_u64_uniform (function)
macro_rules! Depcrate_randrand_u64_uniform {
() => {
// Module: crate::rand
// Provides: {"rand_u64_uniform"}
// Dependencies: {}
pub fn rand_u64_uniform (max : u64) -> u64 { let chunk_size = u64 :: MAX / max ; let end_of_last_chunk = chunk_size * max ; let mut r = rand_u64 () ; while r >= end_of_last_chunk { r = rand_u64 () ; } r / chunk_size }
};
}
