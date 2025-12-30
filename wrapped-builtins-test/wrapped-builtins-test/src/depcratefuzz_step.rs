// Generated macro for fuzz_step (function)
macro_rules! Depcratefuzz_step {
() => {
// Module: crate
// Provides: {"fuzz_step"}
// Dependencies: {}
# [doc = " Random fuzzing step. When run several times, it results in excellent fuzzing entropy such as:"] # [doc = " 11110101010101011110111110011111"] # [doc = " 10110101010100001011101011001010"] # [doc = " 1000000000000000"] # [doc = " 10000000000000110111110000001010"] # [doc = " 1111011111111101010101111110101"] # [doc = " 101111111110100000000101000000"] # [doc = " 10000000110100000000100010101"] # [doc = " 1010101010101000"] fn fuzz_step < I : Int > (rng : & mut Xoshiro128StarStar , x : & mut I) { let ones = ! I :: ZERO ; let bit_indexing_mask : u32 = I :: BITS - 1 ; let rng32 = rng . next_u32 () ; let r0 = bit_indexing_mask & rng32 ; let r1 = bit_indexing_mask & (rng32 >> 7) ; let mask = ones . wrapping_shl (r0) . rotate_left (r1) ; match (rng32 >> 14) % 4 { 0 => * x |= mask , 1 => * x &= mask , _ => * x ^= mask , } let mut alt_ones = I :: ONE ; for _ in 0 .. (I :: BITS / 2) { alt_ones <<= 2 ; alt_ones |= I :: ONE ; } let r0 = bit_indexing_mask & (rng32 >> 16) ; let r1 = bit_indexing_mask & (rng32 >> 23) ; let mask = alt_ones . wrapping_shl (r0) . rotate_left (r1) ; match rng32 >> 30 { 0 => * x |= mask , 1 => * x &= mask , _ => * x ^= mask , } }
};
}
