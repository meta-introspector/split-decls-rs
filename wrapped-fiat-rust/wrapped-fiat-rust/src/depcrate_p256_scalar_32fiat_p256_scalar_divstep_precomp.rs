// Generated macro for fiat_p256_scalar_divstep_precomp (function)
macro_rules! Depcrate_p256_scalar_32fiat_p256_scalar_divstep_precomp {
() => {
// Module: crate::p256_scalar_32
// Provides: {"fiat_p256_scalar_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_p256_scalar_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p256_scalar_divstep_precomp (mut out1 : & mut [u32 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0xb7fcfbb5 ; * IndexConst (& mut out1) . index_mut (1) = 0xd739262f ; * IndexConst (& mut out1) . index_mut (2) = 0x20074414 ; * IndexConst (& mut out1) . index_mut (3) = 0x8ac6f75d ; * IndexConst (& mut out1) . index_mut (4) = 0xb5e3c256 ; * IndexConst (& mut out1) . index_mut (5) = 0xc67428bf ; * IndexConst (& mut out1) . index_mut (6) = 0xeda7aedf ; * IndexConst (& mut out1) . index_mut (7) = 0x444962f2 ; }
};
}
