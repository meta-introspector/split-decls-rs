// Generated macro for new_rand (function)
macro_rules! Depcratenew_rand {
() => {
// Module: crate
// Provides: {"new_rand"}
// Dependencies: {}
# [doc = " New random `Pubkey` for tests and benchmarks."] # [cfg (all (feature = "rand" , not (target_os = "solana")))] pub fn new_rand () -> Pubkey { Pubkey :: from (rand :: random :: < [u8 ; PUBKEY_BYTES] > ()) }
};
}
