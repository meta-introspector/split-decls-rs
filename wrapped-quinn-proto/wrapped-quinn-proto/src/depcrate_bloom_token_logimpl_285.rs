// Generated macro for impl_285 (impl)
macro_rules! Depcrate_bloom_token_logimpl_285 {
() => {
// Module: crate::bloom_token_log
// Provides: {"impl_285"}
// Dependencies: {}
impl BloomTokenLog { # [doc = " Construct with an approximate maximum memory usage and expected number of validation token"] # [doc = " usages per expiration period"] # [doc = ""] # [doc = " Calculates the optimal bloom filter k number automatically."] pub fn new_expected_items (max_bytes : usize , expected_hits : u64) -> Self { Self :: new (max_bytes , optimal_k_num (max_bytes , expected_hits)) } # [doc = " Construct with an approximate maximum memory usage and a [bloom filter k number][bloom]"] # [doc = ""] # [doc = " [bloom]: https://en.wikipedia.org/wiki/Bloom_filter"] # [doc = ""] # [doc = " If choosing a custom k number, note that `BloomTokenLog` always maintains two filters"] # [doc = " between them and divides the allocation budget of `max_bytes` evenly between them. As such,"] # [doc = " each bloom filter will contain `max_bytes * 4` bits."] pub fn new (max_bytes : usize , k_num : u32) -> Self { Self (Mutex :: new (State { config : FilterConfig { filter_max_bytes : max_bytes / 2 , k_num , } , period_1_start : UNIX_EPOCH , filter_1 : Filter :: default () , filter_2 : Filter :: default () , })) } }
};
}
