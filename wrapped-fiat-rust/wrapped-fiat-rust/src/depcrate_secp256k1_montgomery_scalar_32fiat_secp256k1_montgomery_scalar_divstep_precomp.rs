// Generated macro for fiat_secp256k1_montgomery_scalar_divstep_precomp (function)
macro_rules! Depcrate_secp256k1_montgomery_scalar_32fiat_secp256k1_montgomery_scalar_divstep_precomp {
() => {
// Module: crate::secp256k1_montgomery_scalar_32
// Provides: {"fiat_secp256k1_montgomery_scalar_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_scalar_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_divstep_precomp (mut out1 : & mut [u32 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0x2b9cb4e9 ; * IndexConst (& mut out1) . index_mut (1) = 0xd7431a4d ; * IndexConst (& mut out1) . index_mut (2) = 0x32d9c503 ; * IndexConst (& mut out1) . index_mut (3) = 0xab67d35a ; * IndexConst (& mut out1) . index_mut (4) = 0x859ce35f ; * IndexConst (& mut out1) . index_mut (5) = 0xadf6c7e5 ; * IndexConst (& mut out1) . index_mut (6) = 0x1df6c379 ; * IndexConst (& mut out1) . index_mut (7) = 0x61544145 ; }
};
}
