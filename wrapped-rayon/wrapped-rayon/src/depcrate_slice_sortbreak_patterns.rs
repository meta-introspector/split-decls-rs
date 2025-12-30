// Generated macro for break_patterns (function)
macro_rules! Depcrate_slice_sortbreak_patterns {
() => {
// Module: crate::slice::sort
// Provides: {"break_patterns"}
// Dependencies: {}
# [doc = " Scatters some elements around in an attempt to break patterns that might cause imbalanced"] # [doc = " partitions in quicksort."] # [cold] fn break_patterns < T > (v : & mut [T]) { let len = v . len () ; if len >= 8 { let mut seed = len ; let mut gen_usize = | | { if usize :: BITS <= 32 { let mut r = seed as u32 ; r ^= r << 13 ; r ^= r >> 17 ; r ^= r << 5 ; seed = r as usize ; seed } else { let mut r = seed as u64 ; r ^= r << 13 ; r ^= r >> 7 ; r ^= r << 17 ; seed = r as usize ; seed } } ; let modulus = len . next_power_of_two () ; let pos = len / 4 * 2 ; for i in 0 .. 3 { let mut other = gen_usize () & (modulus - 1) ; if other >= len { other -= len ; } v . swap (pos - 1 + i , other) ; } } }
};
}
