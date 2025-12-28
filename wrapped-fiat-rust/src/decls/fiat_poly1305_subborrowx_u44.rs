macro_rules! fiat_poly1305_subborrowx_u44 {
    () => {
        # [doc = " The function fiat_poly1305_subborrowx_u44 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^44"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^44⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xfffffffffff]"] # [doc = "   arg3: [0x0 ~> 0xfffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xfffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_poly1305_subborrowx_u44 (out1 : & mut u64 , out2 : & mut fiat_poly1305_u1 , arg1 : fiat_poly1305_u1 , arg2 : u64 , arg3 : u64) { let x1 : i64 = ((((((arg2 as i128) - (arg1 as i128)) as i64) as i128) - (arg3 as i128)) as i64) ; let x2 : fiat_poly1305_i1 = ((x1 >> 44) as fiat_poly1305_i1) ; let x3 : u64 = (((x1 as i128) & (0xfffffffffff as i128)) as u64) ; * out1 = x3 ; * out2 = (((0x0 as fiat_poly1305_i2) - (x2 as fiat_poly1305_i2)) as fiat_poly1305_u1) ; }
    };
}

fiat_poly1305_subborrowx_u44!();