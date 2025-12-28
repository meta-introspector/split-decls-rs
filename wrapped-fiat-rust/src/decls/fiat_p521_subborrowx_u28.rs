macro_rules! fiat_p521_subborrowx_u28 {
    () => {
        # [doc = " The function fiat_p521_subborrowx_u28 is a subtraction with borrow."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (-arg1 + arg2 + -arg3) mod 2^28"] # [doc = "   out2 = -⌊(-arg1 + arg2 + -arg3) / 2^28⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xfffffff]"] # [doc = "   arg3: [0x0 ~> 0xfffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xfffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p521_subborrowx_u28 (out1 : & mut u32 , out2 : & mut fiat_p521_u1 , arg1 : fiat_p521_u1 , arg2 : u32 , arg3 : u32) { let x1 : i32 = ((((((arg2 as i64) - (arg1 as i64)) as i32) as i64) - (arg3 as i64)) as i32) ; let x2 : fiat_p521_i1 = ((x1 >> 28) as fiat_p521_i1) ; let x3 : u32 = (((x1 as i64) & (0xfffffff as i64)) as u32) ; * out1 = x3 ; * out2 = (((0x0 as fiat_p521_i2) - (x2 as fiat_p521_i2)) as fiat_p521_u1) ; }
    };
}

fiat_p521_subborrowx_u28!();