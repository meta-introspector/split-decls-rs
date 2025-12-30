// Generated macro for fiat_25519_subborrowx_u51 (function)
macro_rules! Depcrate_curve25519_64fiat_25519_subborrowx_u51 {
() => {
// Module: crate::curve25519_64
// Provides: {"fiat_25519_subborrowx_u51"}
// Dependencies: {}
# [doc = " The function fiat_25519_subborrowx_u51 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^51"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^51⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x7ffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0x7ffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x7ffffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_25519_subborrowx_u51 (out1 : & mut u64 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u64 , arg3 : u64) { let x1 : i64 = ((((((arg2 as i128) - (arg1 as i128)) as i64) as i128) - (arg3 as i128)) as i64) ; let x2 : fiat_25519_i1 = ((x1 >> 51) as fiat_25519_i1) ; let x3 : u64 = (((x1 as i128) & (0x7ffffffffffff as i128)) as u64) ; * out1 = x3 ; * out2 = (((0x0 as fiat_25519_i2) - (x2 as fiat_25519_i2)) as fiat_25519_u1) ; }
};
}
