// Generated macro for fiat_25519_scalar_divstep_precomp (function)
macro_rules! Depcrate_curve25519_scalar_64fiat_25519_scalar_divstep_precomp {
() => {
// Module: crate::curve25519_scalar_64
// Provides: {"fiat_25519_scalar_divstep_precomp"}
// Dependencies: {}
# [doc = " The function fiat_25519_scalar_divstep_precomp returns the precomputed value for Bernstein-Yang-inversion (in montgomery form)."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) = ⌊(m - 1) / 2⌋^(if ⌊log2 m⌋ + 1 < 46 then ⌊(49 * (⌊log2 m⌋ + 1) + 80) / 17⌋ else ⌊(49 * (⌊log2 m⌋ + 1) + 57) / 17⌋)"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_25519_scalar_divstep_precomp (mut out1 : & mut [u64 ; 4]) { * IndexConst (& mut out1) . index_mut (0) = 0xd70af84436a7cb92 ; * IndexConst (& mut out1) . index_mut (1) = 0x5f71c978b0b8b159 ; * IndexConst (& mut out1) . index_mut (2) = 0xe76d816974947f1a ; * IndexConst (& mut out1) . index_mut (3) = 0x19a2d36f193e4ff ; }
};
}
