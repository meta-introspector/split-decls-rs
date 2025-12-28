macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_secp256k1_montgomery_divstep_precomp {
    () => {
        deps!();
        # [doc = " The function fiat_secp256k1_montgomery_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_divstep_precomp (mut out1 : & mut [u64 ; 4]) { * IndexConst (& mut out1) . index_mut (0) = 0xf201a41831525e0a ; * IndexConst (& mut out1) . index_mut (1) = 0x9953f9ddcd648d85 ; * IndexConst (& mut out1) . index_mut (2) = 0xe86029463db210a9 ; * IndexConst (& mut out1) . index_mut (3) = 0x24fb8a3104b03709 ; }
    };
}

fiat_secp256k1_montgomery_divstep_precomp!()