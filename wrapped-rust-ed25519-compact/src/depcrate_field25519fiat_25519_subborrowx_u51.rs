// Generated macro for fiat_25519_subborrowx_u51 (function)
macro_rules! Depcrate_field25519fiat_25519_subborrowx_u51 {
() => {
// Module: crate::field25519
// Provides: {"fiat_25519_subborrowx_u51"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_subborrowx_u51 (out1 : & mut u64 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u64 , arg3 : u64 ,) { let x1 : i64 = ((((((arg2 as i128) . wrapping_sub ((arg1 as i128))) as i64) as i128) . wrapping_sub ((arg3 as i128))) as i64) ; let x2 : fiat_25519_i1 = ((x1 >> 51) as fiat_25519_i1) ; let x3 : u64 = (((x1 as i128) & 0x7ffffffffffff_i128) as u64) ; * out1 = x3 ; * out2 = ((0x0_i8 . wrapping_sub ((x2 as fiat_25519_i2))) as fiat_25519_u1) ; }
};
}
