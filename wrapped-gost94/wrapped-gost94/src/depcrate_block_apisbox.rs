// Generated macro for sbox (function)
macro_rules! Depcrate_block_apisbox {
() => {
// Module: crate::block_api
// Provides: {"sbox"}
// Dependencies: {}
fn sbox (a : u32 , s : & SBox) -> u32 { let mut v = 0 ; # [allow (clippy :: needless_range_loop)] for i in 0 .. 8 { let shift = 4 * i ; let k = ((a & (0b1111u32 << shift)) >> shift) as usize ; v += u32 :: from (s [i] [k]) << shift ; } v }
};
}
