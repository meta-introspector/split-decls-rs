// Generated macro for fiat_p256_divstep_precomp (function)
macro_rules! Depcrate_p256_32fiat_p256_divstep_precomp {
() => {
// Module: crate::p256_32
// Provides: {"fiat_p256_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_p256_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p256_divstep_precomp (mut out1 : & mut [u32 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0xb8000000 ; * IndexConst (& mut out1) . index_mut (1) = 0x67ffffff ; * IndexConst (& mut out1) . index_mut (2) = 0x38000000 ; * IndexConst (& mut out1) . index_mut (3) = 0xc0000000 ; * IndexConst (& mut out1) . index_mut (4) = 0x7fffffff ; * IndexConst (& mut out1) . index_mut (5) = 0xd8000000 ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0x2fffffff ; }
};
}
