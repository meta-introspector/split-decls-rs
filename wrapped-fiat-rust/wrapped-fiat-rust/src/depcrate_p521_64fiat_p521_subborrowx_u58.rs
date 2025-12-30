// Generated macro for fiat_p521_subborrowx_u58 (function)
macro_rules! Depcrate_p521_64fiat_p521_subborrowx_u58 {
() => {
// Module: crate::p521_64
// Provides: {"fiat_p521_subborrowx_u58"}
// Dependencies: {}
# [doc = " The function fiat_p521_subborrowx_u58 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^58"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^58⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x3ffffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0x3ffffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x3ffffffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p521_subborrowx_u58 (out1 : & mut u64 , out2 : & mut fiat_p521_u1 , arg1 : fiat_p521_u1 , arg2 : u64 , arg3 : u64) { let x1 : i64 = ((((((arg2 as i128) - (arg1 as i128)) as i64) as i128) - (arg3 as i128)) as i64) ; let x2 : fiat_p521_i1 = ((x1 >> 58) as fiat_p521_i1) ; let x3 : u64 = (((x1 as i128) & (0x3ffffffffffffff as i128)) as u64) ; * out1 = x3 ; * out2 = (((0x0 as fiat_p521_i2) - (x2 as fiat_p521_i2)) as fiat_p521_u1) ; }
};
}
