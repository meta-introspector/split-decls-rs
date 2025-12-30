// Generated macro for read_u32le (function)
macro_rules! Depcrate_gutsread_u32le {
() => {
// Module: crate::guts
// Provides: {"read_u32le"}
// Dependencies: {}
fn read_u32le (xs : & [u8]) -> u32 { assert_eq ! (xs . len () , 4) ; u32 :: from (xs [0]) | (u32 :: from (xs [1]) << 8) | (u32 :: from (xs [2]) << 16) | (u32 :: from (xs [3]) << 24) }
};
}
