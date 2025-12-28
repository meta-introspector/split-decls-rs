macro_rules! fiat_secp256k1_montgomery_scalar_mulx_u64 {
    () => {
        # [doc = " The function fiat_secp256k1_montgomery_scalar_mulx_u64 is a multiplication, returning the full double-width result."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = (arg1 * arg2) mod 2^64"] # [doc = "   out2 = ⌊arg1 * arg2 / 2^64⌋"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [0x0 ~> 0xffffffffffffffff]"] # [doc = "   arg2: [0x0 ~> 0xffffffffffffffff]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffffff]"] # [doc = "   out2: [0x0 ~> 0xffffffffffffffff]"] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_mulx_u64 (out1 : & mut u64 , out2 : & mut u64 , arg1 : u64 , arg2 : u64) { let x1 : u128 = ((arg1 as u128) * (arg2 as u128)) ; let x2 : u64 = ((x1 & (0xffffffffffffffff as u128)) as u64) ; let x3 : u64 = ((x1 >> 64) as u64) ; * out1 = x2 ; * out2 = x3 ; }
    };
}

fiat_secp256k1_montgomery_scalar_mulx_u64!();