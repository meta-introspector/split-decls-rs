macro_rules! fiat_p448_addcarryx_u28 {
    () => {
        # [doc = " The function fiat_p448_addcarryx_u28 is an addition with carry."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 + arg2 + arg3) mod 2^28"] # [doc = "   out2 = ⌊(arg1 + arg2 + arg3) / 2^28⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0x1]"] # [doc = "   arg2: [0x0 ~> 0xfffffff]"] # [doc = "   arg3: [0x0 ~> 0xfffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xfffffff]"] # [doc = "   out2: [0x0 ~> 0x1]"] # [inline] pub const fn fiat_p448_addcarryx_u28 (out1 : & mut u32 , out2 : & mut fiat_p448_u1 , arg1 : fiat_p448_u1 , arg2 : u32 , arg3 : u32) { let x1 : u32 = (((arg1 as u32) + arg2) + arg3) ; let x2 : u32 = (x1 & 0xfffffff) ; let x3 : fiat_p448_u1 = ((x1 >> 28) as fiat_p448_u1) ; * out1 = x2 ; * out2 = x3 ; }
    };
}

fiat_p448_addcarryx_u28!();