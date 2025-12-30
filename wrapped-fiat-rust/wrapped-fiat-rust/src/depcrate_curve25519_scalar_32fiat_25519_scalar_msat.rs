// Generated macro for fiat_25519_scalar_msat (function)
macro_rules! Depcrate_curve25519_scalar_32fiat_25519_scalar_msat {
() => {
// Module: crate::curve25519_scalar_32
// Provides: {"fiat_25519_scalar_msat"}
// Dependencies: {}
# [doc = " The function fiat_25519_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_25519_scalar_msat (mut out1 : & mut [u32 ; 9]) { * IndexConst (& mut out1) . index_mut (0) = 0x5cf5d3ed ; * IndexConst (& mut out1) . index_mut (1) = 0x5812631a ; * IndexConst (& mut out1) . index_mut (2) = 0xa2f79cd6 ; * IndexConst (& mut out1) . index_mut (3) = 0x14def9de ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (5) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (6) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (7) = 0x10000000 ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; }
};
}
