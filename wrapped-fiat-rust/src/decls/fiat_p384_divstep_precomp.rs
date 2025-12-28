macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p384_divstep_precomp {
    () => {
        deps!();
        # [doc = " The function fiat_p384_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_p384_divstep_precomp (mut out1 : & mut [u64 ; 6]) { * IndexConst (& mut out1) . index_mut (0) = 0xfff69400fff18fff ; * IndexConst (& mut out1) . index_mut (1) = 0x2b7feffffd3ff ; * IndexConst (& mut out1) . index_mut (2) = 0xfffedbfffffe97ff ; * IndexConst (& mut out1) . index_mut (3) = 0x2840000002fff ; * IndexConst (& mut out1) . index_mut (4) = 0x6040000050400 ; * IndexConst (& mut out1) . index_mut (5) = 0xfffc480000038000 ; }
    };
}

fiat_p384_divstep_precomp!()