// Generated macro for xor_16 (function)
macro_rules! Depcrate_bb_bytesxor_16 {
() => {
// Module: crate::bb::bytes
// Provides: {"xor_16"}
// Dependencies: {}
pub (crate) fn xor_16 (a : [u8 ; 16] , b : [u8 ; 16]) -> [u8 ; 16] { let a = u128 :: from_ne_bytes (a) ; let b = u128 :: from_ne_bytes (b) ; let r = a ^ b ; r . to_ne_bytes () }
};
}
