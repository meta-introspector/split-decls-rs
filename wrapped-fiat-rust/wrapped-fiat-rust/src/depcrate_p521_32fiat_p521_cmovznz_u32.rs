// Generated macro for fiat_p521_cmovznz_u32 (function)
macro_rules! Depcrate_p521_32fiat_p521_cmovznz_u32 {
() => {
// Module: crate::p521_32
// Provides: {"fiat_p521_cmovznz_u32"}
// Dependencies: {}
# [doc = " The function fiat_p521_cmovznz_u32 is a single-word conditional move."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (if arg1 = 0 then arg2 else arg3)"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffff]"] # [inline] pub const fn fiat_p521_cmovznz_u32 (out1 : & mut u32 , arg1 : fiat_p521_u1 , arg2 : u32 , arg3 : u32) { let x1 : fiat_p521_u1 = (! (! arg1)) ; let x2 : u32 = ((((((0x0 as fiat_p521_i2) - (x1 as fiat_p521_i2)) as fiat_p521_i1) as i64) & (0xffffffff as i64)) as u32) ; let x3 : u32 = ((x2 & arg3) | ((! x2) & arg2)) ; * out1 = x3 ; }
};
}
