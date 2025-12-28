macro_rules! fiat_25519_subborrowx_u25 {
    () => {
        # [doc = " The function fiat_25519_subborrowx_u25 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^25"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^25⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0x1ffffff]"] # [doc = "   arg3: [0x0 ~> 0x1ffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0x1ffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_25519_subborrowx_u25 (out1 : & mut u32 , out2 : & mut fiat_25519_u1 , arg1 : fiat_25519_u1 , arg2 : u32 , arg3 : u32) { let x1 : i32 = ((((((arg2 as i64) - (arg1 as i64)) as i32) as i64) - (arg3 as i64)) as i32) ; let x2 : fiat_25519_i1 = ((x1 >> 25) as fiat_25519_i1) ; let x3 : u32 = (((x1 as i64) & (0x1ffffff as i64)) as u32) ; * out1 = x3 ; * out2 = (((0x0 as fiat_25519_i2) - (x2 as fiat_25519_i2)) as fiat_25519_u1) ; }
    };
}

fiat_25519_subborrowx_u25!()