// Generated macro for fiat_25519_addcarryx_u51 (function)
macro_rules! Depcrate_field25519fiat_25519_addcarryx_u51 {
() => {
// Module: crate::field25519
// Provides: {"fiat_25519_addcarryx_u51"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_addcarryx_u51 (out1 : & mut u64 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u64 , arg3 : u64 ,) { let x1 : u64 = (((arg1 as u64) . wrapping_add (arg2)) . wrapping_add (arg3)) ; let x2 : u64 = (x1 & 0x7ffffffffffff) ; let x3 : fiat_25519_u1 = ((x1 >> 51) as fiat_25519_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
