// Generated macro for impl_287 (impl)
macro_rules! Depcrate_bloom_token_logimpl_287 {
() => {
// Module: crate::bloom_token_log
// Provides: {"impl_287"}
// Dependencies: {}
# [doc = " Default to 20 MiB max memory consumption and expected one million hits"] # [doc = ""] # [doc = " With the default validation token lifetime of 2 weeks, this corresponds to one token usage per"] # [doc = " 1.21 seconds."] impl Default for BloomTokenLog { fn default () -> Self { Self :: new_expected_items (DEFAULT_MAX_BYTES , DEFAULT_EXPECTED_HITS) } }
};
}
