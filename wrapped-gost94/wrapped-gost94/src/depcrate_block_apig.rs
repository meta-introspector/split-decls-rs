// Generated macro for g (function)
macro_rules! Depcrate_block_apig {
() => {
// Module: crate::block_api
// Provides: {"g"}
// Dependencies: {}
fn g (a : u32 , k : u32 , s : & SBox) -> u32 { sbox (a . wrapping_add (k) , s) . rotate_left (11) }
};
}
