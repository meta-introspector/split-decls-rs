macro_rules! fiat_p448_subborrowx_u56 {
    () => {
        # [doc = " The function fiat_p448_subborrowx_u56 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^56"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^56⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p448_subborrowx_u56 (out1 : & mut u64 , out2 : & mut fiat_p448_u1 , arg1 : fiat_p448_u1 , arg2 : u64 , arg3 : u64) { let x1 : i64 = ((((((arg2 as i128) - (arg1 as i128)) as i64) as i128) - (arg3 as i128)) as i64) ; let x2 : fiat_p448_i1 = ((x1 >> 56) as fiat_p448_i1) ; let x3 : u64 = (((x1 as i128) & (0xffffffffffffff as i128)) as u64) ; * out1 = x3 ; * out2 = (((0x0 as fiat_p448_i2) - (x2 as fiat_p448_i2)) as fiat_p448_u1) ; }
    };
}

fiat_p448_subborrowx_u56!();