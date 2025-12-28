macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p384_scalar_divstep_precomp {
    () => {
        deps!();
        # [doc = " The function fiat_p384_scalar_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_p384_scalar_divstep_precomp (mut out1 : & mut [u64 ; 6]) { * IndexConst (& mut out1) . index_mut (0) = 0x49589ae0e6045b6a ; * IndexConst (& mut out1) . index_mut (1) = 0x3c9a5352870040ed ; * IndexConst (& mut out1) . index_mut (2) = 0xdacb097e977dc242 ; * IndexConst (& mut out1) . index_mut (3) = 0xb5ab30a6d1ecbe36 ; * IndexConst (& mut out1) . index_mut (4) = 0x97d7a1081f959973 ; * IndexConst (& mut out1) . index_mut (5) = 0x2ba012f8d27192bc ; }
    };
}

fiat_p384_scalar_divstep_precomp!();