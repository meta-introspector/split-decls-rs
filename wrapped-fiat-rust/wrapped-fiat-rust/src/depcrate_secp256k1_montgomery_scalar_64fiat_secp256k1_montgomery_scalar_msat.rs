// Generated macro for fiat_secp256k1_montgomery_scalar_msat (function)
macro_rules! Depcrate_secp256k1_montgomery_scalar_64fiat_secp256k1_montgomery_scalar_msat {
() => {
// Module: crate::secp256k1_montgomery_scalar_64
// Provides: {"fiat_secp256k1_montgomery_scalar_msat"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_msat (mut out1 : & mut [u64 ; 5]) { * IndexConst (& mut out1) . index_mut (0) = 0xbfd25e8cd0364141 ; * IndexConst (& mut out1) . index_mut (1) = 0xbaaedce6af48a03b ; * IndexConst (& mut out1) . index_mut (2) = 0xfffffffffffffffe ; * IndexConst (& mut out1) . index_mut (3) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u64) ; }
};
}
