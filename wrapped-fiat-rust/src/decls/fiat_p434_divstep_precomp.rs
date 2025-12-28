macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p434_divstep_precomp {
    () => {
        deps!();
        # [doc = " The function fiat_p434_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_p434_divstep_precomp (mut out1 : & mut [u64 ; 7]) { * IndexConst (& mut out1) . index_mut (0) = 0x9f9776e27e1a2b72 ; * IndexConst (& mut out1) . index_mut (1) = 0x28b59f067e2393d0 ; * IndexConst (& mut out1) . index_mut (2) = 0xcf316ce1572add54 ; * IndexConst (& mut out1) . index_mut (3) = 0x312c8965f9032c2f ; * IndexConst (& mut out1) . index_mut (4) = 0x9d9cab29ad90d34c ; * IndexConst (& mut out1) . index_mut (5) = 0x6e1ddae1d9609ae1 ; * IndexConst (& mut out1) . index_mut (6) = 0x6df82285eec6 ; }
    };
}

fiat_p434_divstep_precomp!();