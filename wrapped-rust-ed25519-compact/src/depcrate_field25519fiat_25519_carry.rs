// Generated macro for fiat_25519_carry (function)
macro_rules! Depcrate_field25519fiat_25519_carry {
() => {
// Module: crate::field25519
// Provides: {"fiat_25519_carry"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_carry (out1 : & mut [u64 ; 5] , arg1 : & [u64 ; 5]) { let x1 : u64 = (arg1 [0]) ; let x2 : u64 = ((x1 >> 51) . wrapping_add ((arg1 [1]))) ; let x3 : u64 = ((x2 >> 51) . wrapping_add ((arg1 [2]))) ; let x4 : u64 = ((x3 >> 51) . wrapping_add ((arg1 [3]))) ; let x5 : u64 = ((x4 >> 51) . wrapping_add ((arg1 [4]))) ; let x6 : u64 = ((x1 & 0x7ffffffffffff) . wrapping_add (((x5 >> 51) . wrapping_mul (0x13)))) ; let x7 : u64 = ((((x6 >> 51) as fiat_25519_u1) as u64) . wrapping_add ((x2 & 0x7ffffffffffff))) ; let x8 : u64 = (x6 & 0x7ffffffffffff) ; let x9 : u64 = (x7 & 0x7ffffffffffff) ; let x10 : u64 = ((((x7 >> 51) as fiat_25519_u1) as u64) . wrapping_add ((x3 & 0x7ffffffffffff))) ; let x11 : u64 = (x4 & 0x7ffffffffffff) ; let x12 : u64 = (x5 & 0x7ffffffffffff) ; out1 [0] = x8 ; out1 [1] = x9 ; out1 [2] = x10 ; out1 [3] = x11 ; out1 [4] = x12 ; }
};
}
