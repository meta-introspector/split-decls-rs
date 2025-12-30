// Generated macro for fiat_secp256k1_montgomery_divstep_precomp (function)
macro_rules! Depcrate_secp256k1_montgomery_32fiat_secp256k1_montgomery_divstep_precomp {
() => {
// Module: crate::secp256k1_montgomery_32
// Provides: {"fiat_secp256k1_montgomery_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_divstep_precomp (mut out1 : & mut [u32 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0x31525e0a ; * IndexConst (& mut out1) . index_mut (1) = 0xf201a418 ; * IndexConst (& mut out1) . index_mut (2) = 0xcd648d85 ; * IndexConst (& mut out1) . index_mut (3) = 0x9953f9dd ; * IndexConst (& mut out1) . index_mut (4) = 0x3db210a9 ; * IndexConst (& mut out1) . index_mut (5) = 0xe8602946 ; * IndexConst (& mut out1) . index_mut (6) = 0x4b03709 ; * IndexConst (& mut out1) . index_mut (7) = 0x24fb8a31 ; }
};
}
