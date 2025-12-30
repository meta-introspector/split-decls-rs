// Generated macro for fiat_sm2_divstep_precomp (function)
macro_rules! Depcrate_sm2_32fiat_sm2_divstep_precomp {
() => {
// Module: crate::sm2_32
// Provides: {"fiat_sm2_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_sm2_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_sm2_divstep_precomp (mut out1 : & mut [u32 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0x8ffffffe ; * IndexConst (& mut out1) . index_mut (1) = 0x50000002 ; * IndexConst (& mut out1) . index_mut (2) = 0x9ffffffe ; * IndexConst (& mut out1) . index_mut (3) = 0xe8000000 ; * IndexConst (& mut out1) . index_mut (4) = 0x8ffffffe ; * IndexConst (& mut out1) . index_mut (5) = 0xd0000001 ; * IndexConst (& mut out1) . index_mut (6) = 0x1ffffffd ; * IndexConst (& mut out1) . index_mut (7) = 0x28000001 ; }
};
}
