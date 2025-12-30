// Generated macro for fiat_sm2_msat (function)
macro_rules! Depcrate_sm2_64fiat_sm2_msat {
() => {
// Module: crate::sm2_64
// Provides: {"fiat_sm2_msat"}
// Dependencies: {}
# [doc = " The function fiat_sm2_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_sm2_msat (mut out1 : & mut [u64 ; 5]) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff00000000 ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (3) = 0xfffffffeffffffff ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u64) ; }
};
}
