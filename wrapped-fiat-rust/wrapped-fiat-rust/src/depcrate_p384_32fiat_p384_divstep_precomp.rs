// Generated macro for fiat_p384_divstep_precomp (function)
macro_rules! Depcrate_p384_32fiat_p384_divstep_precomp {
() => {
// Module: crate::p384_32
// Provides: {"fiat_p384_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_p384_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p384_divstep_precomp (mut out1 : & mut [u32 ; 12]) { * IndexConst (& mut out1) . index_mut (0) = 0xfff18fff ; * IndexConst (& mut out1) . index_mut (1) = 0xfff69400 ; * IndexConst (& mut out1) . index_mut (2) = 0xffffd3ff ; * IndexConst (& mut out1) . index_mut (3) = 0x2b7fe ; * IndexConst (& mut out1) . index_mut (4) = 0xfffe97ff ; * IndexConst (& mut out1) . index_mut (5) = 0xfffedbff ; * IndexConst (& mut out1) . index_mut (6) = 0x2fff ; * IndexConst (& mut out1) . index_mut (7) = 0x28400 ; * IndexConst (& mut out1) . index_mut (8) = 0x50400 ; * IndexConst (& mut out1) . index_mut (9) = 0x60400 ; * IndexConst (& mut out1) . index_mut (10) = 0x38000 ; * IndexConst (& mut out1) . index_mut (11) = 0xfffc4800 ; }
};
}
