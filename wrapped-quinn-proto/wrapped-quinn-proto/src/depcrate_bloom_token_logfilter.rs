// Generated macro for Filter (enum)
macro_rules! Depcrate_bloom_token_logFilter {
() => {
// Module: crate::bloom_token_log
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " Period filter within [`State`]"] enum Filter { Set (HashSet < u64 , IdentityBuildHasher >) , Bloom (BloomFilter < FxBuildHasher >) , }
};
}
