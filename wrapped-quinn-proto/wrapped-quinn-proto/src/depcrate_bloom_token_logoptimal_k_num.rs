// Generated macro for optimal_k_num (function)
macro_rules! Depcrate_bloom_token_logoptimal_k_num {
() => {
// Module: crate::bloom_token_log
// Provides: {"optimal_k_num"}
// Dependencies: {}
fn optimal_k_num (num_bytes : usize , expected_hits : u64) -> u32 { let num_bits = (num_bytes as u64) . saturating_mul (8) ; let expected_hits = expected_hits . max (1) ; (((num_bits as f64 / expected_hits as f64) * LN_2) . round () as u32) . max (1) }
};
}
