macro_rules! fiat_p434_addcarryx_u64 {
    () => {
        # [doc = " The function fiat_p434_addcarryx_u64 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^64"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^64⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xffffffffffffffff]"] # [doc = "   arg3: [0x0 ~> 0xffffffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p434_addcarryx_u64 (out1 : & mut u64 , out2 : & mut fiat_p434_u1 , arg1 : fiat_p434_u1 , arg2 : u64 , arg3 : u64) { let x1 : u128 = (((arg1 as u128) + (arg2 as u128)) + (arg3 as u128)) ; let x2 : u64 = ((x1 & (0xffffffffffffffff as u128)) as u64) ; let x3 : fiat_p434_u1 = ((x1 >> 64) as fiat_p434_u1) ; * out1 = x2 ; * out2 = x3 ; }
    };
}

fiat_p434_addcarryx_u64!();