// Generated macro for fiat_25519_addcarryx_u51 (function)
macro_rules! Depcrate_curve25519_64fiat_25519_addcarryx_u51 {
() => {
// Module: crate::curve25519_64
// Provides: {"fiat_25519_addcarryx_u51"}
// Dependencies: {}
# [doc = " The function fiat_25519_addcarryx_u51 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^51"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^51⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x7ffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0x7ffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x7ffffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_25519_addcarryx_u51 (out1 : & mut u64 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u64 , arg3 : u64) { let x1 : u64 = (((arg1 as u64) + arg2) + arg3) ; let x2 : u64 = (x1 & 0x7ffffffffffff) ; let x3 : fiat_25519_u1 = ((x1 >> 51) as fiat_25519_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
