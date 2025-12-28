macro_rules! fiat_secp256k1_montgomery_scalar_mulx_u32 {
    () => {
        # [doc = " The function fiat_secp256k1_montgomery_scalar_mulx_u32 is a multiplication, returning the full double-width result."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 * arg2) mod 2^32"] # [doc = "   out2 = ⌊arg1 * arg2 / 2^32⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0xffffffff]"] # [doc = "   arg2: [0x0 ~> 0xffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffff]"] # [doc = "   out2: [0x0 ~> 0xffffffff]"] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_mulx_u32 (out1 : & mut u32 , out2 : & mut u32 , arg1 : u32 , arg2 : u32) { let x1 : u64 = ((arg1 as u64) * (arg2 as u64)) ; let x2 : u32 = ((x1 & (0xffffffff as u64)) as u32) ; let x3 : u32 = ((x1 >> 32) as u32) ; * out1 = x2 ; * out2 = x3 ; }
    };
}

fiat_secp256k1_montgomery_scalar_mulx_u32!()