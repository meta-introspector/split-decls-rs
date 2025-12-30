// Generated macro for hash_to_partition (function)
macro_rules! Depcratehash_to_partition {
() => {
// Module: crate
// Provides: {"hash_to_partition"}
// Dependencies: {}
# [doc = " Compute the partition index by modulo the address hash to number of partitions w.o bias."] # [doc = " (rand_int * DESIRED_RANGE_MAX) / (RAND_MAX + 1)"] # [allow (clippy :: arithmetic_side_effects)] fn hash_to_partition (hash : u64 , partitions : usize) -> usize { ((partitions as u128) . saturating_mul (u128 :: from (hash)) . saturating_div (u128 :: from (u64 :: MAX) . saturating_add (1))) as usize }
};
}
