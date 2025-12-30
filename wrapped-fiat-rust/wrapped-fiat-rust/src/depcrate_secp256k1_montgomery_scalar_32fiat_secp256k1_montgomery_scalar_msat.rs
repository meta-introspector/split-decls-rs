// Generated macro for fiat_secp256k1_montgomery_scalar_msat (function)
macro_rules! Depcrate_secp256k1_montgomery_scalar_32fiat_secp256k1_montgomery_scalar_msat {
() => {
// Module: crate::secp256k1_montgomery_scalar_32
// Provides: {"fiat_secp256k1_montgomery_scalar_msat"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_msat (mut out1 : & mut [u32 ; 9]) { * IndexConst (& mut out1) . index_mut (0) = 0xd0364141 ; * IndexConst (& mut out1) . index_mut (1) = 0xbfd25e8c ; * IndexConst (& mut out1) . index_mut (2) = 0xaf48a03b ; * IndexConst (& mut out1) . index_mut (3) = 0xbaaedce6 ; * IndexConst (& mut out1) . index_mut (4) = 0xfffffffe ; * IndexConst (& mut out1) . index_mut (5) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; }
};
}
