// Generated macro for fiat_poly1305_addcarryx_u43 (function)
macro_rules! Depcrate_poly1305_64fiat_poly1305_addcarryx_u43 {
() => {
// Module: crate::poly1305_64
// Provides: {"fiat_poly1305_addcarryx_u43"}
// Dependencies: {}
# [doc = " The function fiat_poly1305_addcarryx_u43 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^43"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^43⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x7ffffffffff]"] # [doc = "   arg3: [0x0 ~> 0x7ffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x7ffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_poly1305_addcarryx_u43 (out1 : & mut u64 , out2 : & mut fiat_poly1305_u1 , arg1 : fiat_poly1305_u1 , arg2 : u64 , arg3 : u64) { let x1 : u64 = (((arg1 as u64) + arg2) + arg3) ; let x2 : u64 = (x1 & 0x7ffffffffff) ; let x3 : fiat_poly1305_u1 = ((x1 >> 43) as fiat_poly1305_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
