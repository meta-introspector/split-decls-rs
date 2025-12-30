// Generated macro for fiat_secp256k1_montgomery_addcarryx_u32 (function)
macro_rules! Depcrate_secp256k1_montgomery_32fiat_secp256k1_montgomery_addcarryx_u32 {
() => {
// Module: crate::secp256k1_montgomery_32
// Provides: {"fiat_secp256k1_montgomery_addcarryx_u32"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_addcarryx_u32 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^32"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^32⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_secp256k1_montgomery_addcarryx_u32 (out1 : & mut u32 , out2 : & mut fiat_secp256k1_montgomery_u1 , arg1 : fiat_secp256k1_montgomery_u1 , arg2 : u32 , arg3 : u32) { let x1 : u64 = (((arg1 as u64) + (arg2 as u64)) + (arg3 as u64)) ; let x2 : u32 = ((x1 & (0xffffffff as u64)) as u32) ; let x3 : fiat_secp256k1_montgomery_u1 = ((x1 >> 32) as fiat_secp256k1_montgomery_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
