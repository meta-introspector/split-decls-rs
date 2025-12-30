// Generated macro for BloomTokenLog (struct)
macro_rules! Depcrate_bloom_token_logBloomTokenLog {
() => {
// Module: crate::bloom_token_log
// Provides: {"BloomTokenLog"}
// Dependencies: {}
# [doc = " Bloom filter-based [`TokenLog`]"] # [doc = ""] # [doc = " Parameterizable over an approximate maximum number of bytes to allocate. Starts out by storing"] # [doc = " used tokens in a hash set. Once the hash set becomes too large, converts it to a bloom filter."] # [doc = " This achieves a memory profile of linear growth with an upper bound."] # [doc = ""] # [doc = " Divides time into periods based on `lifetime` and stores two filters at any given moment, for"] # [doc = " each of the two periods currently non-expired tokens could expire in. As such, turns over"] # [doc = " filters as time goes on to avoid bloom filter false positive rate increasing infinitely over"] # [doc = " time."] pub struct BloomTokenLog (Mutex < State >) ;
};
}
