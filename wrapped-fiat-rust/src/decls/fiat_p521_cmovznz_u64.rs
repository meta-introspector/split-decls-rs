macro_rules! fiat_p521_cmovznz_u64 {
    () => {
        # [doc = " The function fiat_p521_cmovznz_u64 is a single-word conditional move."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (if arg1 = 0 then arg2 else arg3)"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffffff]"] # [inline] pub const fn fiat_p521_cmovznz_u64 (out1 : & mut u64 , arg1 : fiat_p521_u1 , arg2 : u64 , arg3 : u64) { let x1 : fiat_p521_u1 = (! (! arg1)) ; let x2 : u64 = ((((((0x0 as fiat_p521_i2) - (x1 as fiat_p521_i2)) as fiat_p521_i1) as i128) & (0xffffffffffffffff as i128)) as u64) ; let x3 : u64 = ((x2 & arg3) | ((! x2) & arg2)) ; * out1 = x3 ; }
    };
}

fiat_p521_cmovznz_u64!()