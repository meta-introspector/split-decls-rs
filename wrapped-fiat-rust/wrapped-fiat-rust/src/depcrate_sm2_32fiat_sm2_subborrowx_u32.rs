// Generated macro for fiat_sm2_subborrowx_u32 (function)
macro_rules! Depcrate_sm2_32fiat_sm2_subborrowx_u32 {
() => {
// Module: crate::sm2_32
// Provides: {"fiat_sm2_subborrowx_u32"}
// Dependencies: {}
# [doc = " The function fiat_sm2_subborrowx_u32 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^32"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^32⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_sm2_subborrowx_u32 (out1 : & mut u32 , out2 : & mut fiat_sm2_u1 , arg1 : fiat_sm2_u1 , arg2 : u32 , arg3 : u32) { let x1 : i64 = (((arg2 as i64) - (arg1 as i64)) - (arg3 as i64)) ; let x2 : fiat_sm2_i1 = ((x1 >> 32) as fiat_sm2_i1) ; let x3 : u32 = ((x1 & (0xffffffff as i64)) as u32) ; * out1 = x3 ; * out2 = (((0x0 as fiat_sm2_i2) - (x2 as fiat_sm2_i2)) as fiat_sm2_u1) ; }
};
}
