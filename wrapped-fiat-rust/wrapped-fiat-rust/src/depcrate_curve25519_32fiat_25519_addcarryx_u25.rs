// Generated macro for fiat_25519_addcarryx_u25 (function)
macro_rules! Depcrate_curve25519_32fiat_25519_addcarryx_u25 {
() => {
// Module: crate::curve25519_32
// Provides: {"fiat_25519_addcarryx_u25"}
// Dependencies: {}
# [doc = " The function fiat_25519_addcarryx_u25 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^25"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^25⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x1ffffff]"] # [doc = "   arg3: [0x0 ~> 0x1ffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x1ffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_25519_addcarryx_u25 (out1 : & mut u32 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u32 , arg3 : u32) { let x1 : u32 = (((arg1 as u32) + arg2) + arg3) ; let x2 : u32 = (x1 & 0x1ffffff) ; let x3 : fiat_25519_u1 = ((x1 >> 25) as fiat_25519_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
