// Generated macro for fiat_p384_scalar_divstep_precomp (function)
macro_rules! Depcrate_p384_scalar_32fiat_p384_scalar_divstep_precomp {
() => {
// Module: crate::p384_scalar_32
// Provides: {"fiat_p384_scalar_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_p384_scalar_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p384_scalar_divstep_precomp (mut out1 : & mut [u32 ; 12]) { * IndexConst (& mut out1) . index_mut (0) = 0xe6045b6a ; * IndexConst (& mut out1) . index_mut (1) = 0x49589ae0 ; * IndexConst (& mut out1) . index_mut (2) = 0x870040ed ; * IndexConst (& mut out1) . index_mut (3) = 0x3c9a5352 ; * IndexConst (& mut out1) . index_mut (4) = 0x977dc242 ; * IndexConst (& mut out1) . index_mut (5) = 0xdacb097e ; * IndexConst (& mut out1) . index_mut (6) = 0xd1ecbe36 ; * IndexConst (& mut out1) . index_mut (7) = 0xb5ab30a6 ; * IndexConst (& mut out1) . index_mut (8) = 0x1f959973 ; * IndexConst (& mut out1) . index_mut (9) = 0x97d7a108 ; * IndexConst (& mut out1) . index_mut (10) = 0xd27192bc ; * IndexConst (& mut out1) . index_mut (11) = 0x2ba012f8 ; }
};
}
