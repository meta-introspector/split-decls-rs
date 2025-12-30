// Generated macro for fiat_p521_addcarryx_u27 (function)
macro_rules! Depcrate_p521_32fiat_p521_addcarryx_u27 {
() => {
// Module: crate::p521_32
// Provides: {"fiat_p521_addcarryx_u27"}
// Dependencies: {}
# [doc = " The function fiat_p521_addcarryx_u27 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^27"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^27⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x7ffffff]"] # [doc = "   arg3: [0x0 ~> 0x7ffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x7ffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p521_addcarryx_u27 (out1 : & mut u32 , out2 : & mut fiat_p521_u1 , arg1 : fiat_p521_u1 , arg2 : u32 , arg3 : u32) { let x1 : u32 = (((arg1 as u32) + arg2) + arg3) ; let x2 : u32 = (x1 & 0x7ffffff) ; let x3 : fiat_p521_u1 = ((x1 >> 27) as fiat_p521_u1) ; * out1 = x2 ; * out2 = x3 ; }
};
}
