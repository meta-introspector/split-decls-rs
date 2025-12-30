// Generated macro for IdentityHasher (struct)
macro_rules! Depcrate_bloom_token_logIdentityHasher {
() => {
// Module: crate::bloom_token_log
// Provides: {"IdentityHasher"}
// Dependencies: {}
# [doc = " Hasher that is the identity operation--it assumes that exactly 8 bytes will be hashed, and the"] # [doc = " resultant hash is those bytes as a `u64`"] # [derive (Default)] struct IdentityHasher { data : [u8 ; 8] , # [cfg (debug_assertions)] wrote_8_byte_slice : bool , }
};
}
