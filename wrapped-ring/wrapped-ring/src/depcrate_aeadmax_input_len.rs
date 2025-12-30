// Generated macro for max_input_len (function)
macro_rules! Depcrate_aeadmax_input_len {
() => {
// Module: crate::aead
// Provides: {"max_input_len"}
// Dependencies: {}
const fn max_input_len (block_len : usize , overhead_blocks_per_nonce : usize) -> usize { usize_from_u64_saturated (((1u64 << 32) - u64_from_usize (overhead_blocks_per_nonce)) * u64_from_usize (block_len) ,) }
};
}
